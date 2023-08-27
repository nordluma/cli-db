use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(Entry),
    Get(Key),
    Remove(Entry),
}

#[derive(Args, Debug)]
pub struct Key {
    key: String,
}

#[derive(Args, Debug)]
pub struct Entry {
    key: String,
    value: String,
}
