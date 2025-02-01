use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use clap::Parser;
use cli::*;
use std::ops::*;
use itertools::Itertools;

mod cli;


#[derive(Clone, Copy)]
struct Alpha(u8);

impl Alpha {
    fn space() -> Self {
        Self(b' ')
    }

    fn from_str(value: &str) -> Vec<Self> {
        value.bytes()
            .into_iter()
            .map(|c| Alpha::from_ascii(c))
            .collect()
    }

    fn to_str(value: &[Self]) -> String {
        let bytes = value.into_iter()
            .map(|c| c.to_ascii())
            .collect();
        String::from_utf8(bytes).unwrap()
    }

    fn from_num(value: i32) -> Self {
        if value >= 0 && value < 26 {
            Self(value as u8 + b'A')
        } else {
            Self::space()
        }
    }

    fn from_ascii(value: u8) -> Self {
        Self(value)
    }

    fn to_num(self) -> Option<i32> {
        let num = self.0 - b'A';
        if num < 26 {
            Some(num as i32)
        } else {
            None
        }
    }

    fn to_ascii(self) -> u8 {
        self.0
    }
}

impl Add<Alpha> for Alpha {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.to_num(), rhs.to_num()) {
            (Some(lhs), Some(rhs)) => Self::from_num((lhs + rhs) % 26),
            _ => Self::space(),
        }
    }
}

impl Sub<Alpha> for Alpha {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.to_num(), rhs.to_num()) {
            (Some(lhs), Some(rhs)) => Self::from_num((lhs - rhs).rem_euclid(26)),
            _ => Self::space(),
        }
    }
}

fn vigenere(key: &[Alpha], text: &mut [Alpha], encrypt: bool) {
    let mut i = 0;

    for c in text.iter_mut() {
        if c.to_ascii() != b' ' {
            let key_c = key[i % key.len()];
            if encrypt {
                *c = *c + key_c;
            } else {
                *c = *c - key_c;
            }
            i += 1;
        }
    }
}

fn run_vigenere_stream(encrypt: bool, key: String) {
    let key = Alpha::from_str(key.as_str());
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let mut line = Alpha::from_str(line.unwrap().as_str());
        vigenere(&key, &mut line, encrypt);
        println!("{}", Alpha::to_str(&line));
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Vigenere { operation } => {
            match operation {
                VigenereOperation::Encrypt { key } => run_vigenere_stream(true, key),
                VigenereOperation::Decrypt { key } => run_vigenere_stream(false, key),
                VigenereOperation::Bruteforce {
                    dictionary_file,
                    max_length,
                    cipher_text,
                } => {
                    let mut dictionary = HashSet::new();
                    let file = std::fs::File::open(dictionary_file).unwrap();
                    let file = BufReader::new(file);
                    for line in file.lines() {
                        dictionary.insert(line.unwrap().to_uppercase());
                    }

                    let mut text = Vec::new();
                    let mut i = 0;
                    for key in (b'A'..b'Z').permutations(max_length) {
                        text.clear();
                        text.extend_from_slice(&Alpha::from_str(cipher_text.as_str()));

                        let key: Vec<Alpha> = key.into_iter().map(Alpha::from_ascii).collect();
                        vigenere(&key, &mut text, false);
                        let words = Alpha::to_str(&text);
                        if words.split(" ").all(|word| dictionary.contains(word)) {
                            println!("\nKey is {}", Alpha::to_str(&key));
                            return;
                        }
                        i += 1;
                        if (i % 1000000) == 0 {
                            print!(".");
                            std::io::stdout().flush().unwrap();
                        }
                        if (i % 80000000) == 0 {
                            println!(" {}", Alpha::to_str(&key));
                        }
                    }
                    println!();
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere() {
        let key = Alpha::from_str("KEY");
        let cases = [
            ("TWOWORDS", "DAMGSPNW"),
            ("TWO WORDS", "DAM GSPNW"),
        ];

        // Ensure we can encrypt these words correctly.
        for (plain_text, cipher_text) in cases {
            let mut text = Alpha::from_str(plain_text);
            vigenere(&key, &mut text, true);
            assert_eq!(Alpha::to_str(&text), cipher_text);
        }

        // Ensure we can decrypt these words correctly.
        for (plain_text, cipher_text) in cases {
            let mut text = Alpha::from_str(cipher_text);
            vigenere(&key, &mut text, false);
            assert_eq!(Alpha::to_str(&text), plain_text);
        }
    }
}
