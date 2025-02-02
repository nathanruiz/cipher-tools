use clap::{Parser, Subcommand};

/// A collection of utilities to assist in breaking simple ciphers, commonly used in ARGs.
#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform operations with vigenere ciphers.
    Vigenere {
        #[clap(subcommand)]
        operation: VigenereOperation,
    },
    /// Manage conversions between A-Z characters and their positions in the alphabet.
    A1Z26 {
        #[clap(subcommand)]
        operation: A1Z26Operation,
    },
    /// Perform a substitution ciphers with a provided lookup file.
    Substitution {
        /// A lookup file with each line containing a mapping between the source
        /// and target characters separated by a space.
        lookup_file: String,

        /// Apply the substitutions backwards. This can be used to decrypt a previously encrypted
        /// message.
        #[clap(short, long, action)]
        reverse: bool,
    },
    /// Perform a Caesar cipher with a provided offset.
    Caesar {
        /// A lookup file with each line containing a mapping between the source and target
        /// characters separated by a space.
        offset: i32,

        /// Apply the substitutions backwards. This can be used to decrypt a previously encrypted
        /// message.
        #[clap(short, long, action)]
        reverse: bool,
    },
    /// Perform an Atbash cipher to the content from stdin.
    Atbash {}
}

#[derive(Subcommand)]
pub enum VigenereOperation {
    /// Decrypt content from stdin with a specified key and write the output stdout.
    Decrypt {
        key: String,
    },
    /// Encrypt content from stdin with a specified key and write the output stdout.
    Encrypt {
        key: String,
    },
    /// Attempt to decrypt cipher text using all the entries in the dictionary. The same dictionary
    /// will be used to determine if the decrypted text is valid or not. The key that results in
    /// the highest number of words in the output will be assumed to be the key. If less than half
    /// of the words exist in the dictionary, the key will be assumed to be wrong.
    Dictionary {
        dictionary_file: String,
        cipher_text: String,
    },
    /// Attempt to decrypt cipher text by brute forcing all keys, up to a certain length. The
    /// provided dictionary will be used to determine if the decrypted text is valid or not. The
    /// key that results in the highest number of words in the output will be assumed to be the
    /// key. If less than half of the words exist in the dictionary, the key will be assumed to be
    /// wrong.
    Bruteforce {
        dictionary_file: String,
        max_length: usize,
        cipher_text: String,
    },
}

#[derive(Subcommand)]
pub enum A1Z26Operation {
    /// Decode text encoded in the A1Z26 format.
    Decode
}
