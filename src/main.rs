use std::io::{BufRead, BufReader, Write};
use std::collections::{HashSet, HashMap};
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

fn find_vigenere_key<KeyIterator>(
    cipher_text: &str,
    keys: KeyIterator,
    dictionary: HashSet<String>
) -> Option<Vec<Alpha>>
    where KeyIterator: Iterator<Item=Vec<Alpha>>
{
    let mut text = Vec::new();
    let mut i = 0;
    let mut closest_key: Option<(Vec<Alpha>, usize)> = None;
    for key in keys {
        text.clear();
        text.extend_from_slice(&Alpha::from_str(cipher_text));

        vigenere(&key, &mut text, false);
        let words = Alpha::to_str(&text);
        let total_words = words.split(" ").count();
        let valid_words = words.split(" ").filter(|word| dictionary.contains(*word)).count();

        // All words returned are in the dictionary, so we can assume this is a perfect match.
        if valid_words == total_words {
            println!();
            return Some(key);
        }

        // At least half of the words match, so might be a candidate for a near match. This helps
        // in cases were some words aren't in the provided dictionary.
        if valid_words > total_words / 2 {
            match &closest_key {
                Some((_, score)) => {
                    if valid_words > *score {
                        closest_key = Some((key.clone(), valid_words));
                    }
                }
                None => closest_key = Some((key.clone(), valid_words)),
            };
        }

        i += 1;
        if (i % 1000) == 0 {
            print!(".");
            std::io::stdout().flush().unwrap();
        }
        if (i % 80000) == 0 {
            println!(" {}", Alpha::to_str(&key));
        }
    }
    println!();
    closest_key.map(|(key, _)| key)
}

fn load_dictionary(dictionary_file: &str) -> HashSet<String> {
    let mut dictionary = HashSet::new();
    let file = std::fs::File::open(dictionary_file).unwrap();
    let file = BufReader::new(file);
    for line in file.lines() {
        dictionary.insert(line.unwrap().to_uppercase());
    }
    dictionary
}

fn a1z26_decode(line: &str) -> Vec<String> {
    line.split(" ")
        .filter(|word| !word.is_empty())
        .map(|word| {
            let word: Vec<Alpha> = word.split("-")
                .map(|s| str::parse(s).unwrap())
                .map(|i: i32| Alpha::from_num(i - 1))
                .collect();
            Alpha::to_str(&word)
        })
        .collect()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Vigenere { operation } => {
            match operation {
                VigenereOperation::Encrypt { key } => run_vigenere_stream(true, key),
                VigenereOperation::Decrypt { key } => run_vigenere_stream(false, key),
                VigenereOperation::Dictionary {
                    dictionary_file,
                    cipher_text,
                } => {
                    let dictionary = load_dictionary(dictionary_file.as_str());

                    let file = std::fs::File::open(dictionary_file).unwrap();
                    let file = BufReader::new(file);
                    let keys = file.lines()
                        .map(|line| line.unwrap().to_uppercase())
                        .map(|key| key.bytes().map(Alpha::from_ascii).collect());

                    match find_vigenere_key(cipher_text.as_str(), keys, dictionary) {
                        Some(key) => {
                            let mut plain_text = Alpha::from_str(cipher_text.as_str());
                            vigenere(&key, &mut plain_text, false);
                            println!("Key is {}: {} -> {}", Alpha::to_str(&key), cipher_text, Alpha::to_str(&plain_text));
                        }
                        None => println!("No key found"),
                    };
                },
                VigenereOperation::Bruteforce {
                    dictionary_file,
                    max_length,
                    cipher_text,
                } => {
                    let dictionary = load_dictionary(dictionary_file.as_str());

                    let keys = (b'A'..b'Z').permutations(max_length)
                        .map(|key| key.into_iter().map(Alpha::from_ascii).collect());

                    match find_vigenere_key(cipher_text.as_str(), keys, dictionary) {
                        Some(key) => {
                            let mut plain_text = Alpha::from_str(cipher_text.as_str());
                            vigenere(&key, &mut plain_text, false);
                            println!("Key is {}: {} -> {}", Alpha::to_str(&key), cipher_text, Alpha::to_str(&plain_text));
                        }
                        None => println!("No key found"),
                    };
                },
            }
        }
        Commands::A1Z26 { operation } => {
            match operation {
                A1Z26Operation::Decode => {
                    let stdin = std::io::stdin();
                    for line in stdin.lock().lines() {
                        let words = a1z26_decode(line.unwrap().as_str());
                        println!("{}", words.join(" "));
                    }
                }
            }
        }
        Commands::Substitution { lookup_file, reverse } => {
            let file = std::fs::File::open(lookup_file).unwrap();
            let file = BufReader::new(file);
            let mapping: HashMap<char, char> = file.lines()
                .map(|line| line.unwrap().to_uppercase())
                .map(|line| {
                    let (src, dst) = line.split_once(" ").unwrap();
                    let src = src.chars().nth(0).unwrap();
                    let dst = dst.chars().nth(0).unwrap();
                    if reverse {
                        (dst, src)
                    } else {
                        (src, dst)
                    }
                })
                .collect();

            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                let line: String = line.unwrap()
                    .chars()
                    .map(|c| {
                        match mapping.get(&c) {
                            Some(c) => *c,
                            None => c,
                        }
                    })
                    .collect();
                println!("{}", line);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere() {
        let cases = [
            ("KEY", "TWOWORDS", "DAMGSPNW"),
            ("KEY", "TWO WORDS", "DAM GSPNW"),
        ];

        // Ensure we can encrypt these words correctly.
        for (key, plain_text, cipher_text) in cases {
            let key = Alpha::from_str(key);
            let mut text = Alpha::from_str(plain_text);
            vigenere(&key, &mut text, true);
            assert_eq!(Alpha::to_str(&text), cipher_text);
        }

        // Ensure we can decrypt these words correctly.
        for (key, plain_text, cipher_text) in cases {
            let key = Alpha::from_str(key);
            let mut text = Alpha::from_str(cipher_text);
            vigenere(&key, &mut text, false);
            assert_eq!(Alpha::to_str(&text), plain_text);
        }
    }

    #[test]
    fn test_a1z26() {
        assert_eq!(a1z26_decode("1-2-3 24-25-26"), vec!["ABC", "XYZ"]);
    }
}
