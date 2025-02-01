use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Vigenere {
        #[clap(subcommand)]
        operation: VigenereOperation,
    },
}

#[derive(Subcommand)]
pub enum VigenereOperation {
    Decrypt {
        key: String,
    },
}
