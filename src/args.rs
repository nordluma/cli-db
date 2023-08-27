use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add an entry to the db
    Add(Entry),
    /// Update an entry
    Update(Entry),
    /// Retrieve a single entry by key
    Get {
        /// Key string
        key: String,
    },
    /// Delete an entry from db by key
    Remove {
        /// Key string
        key: String,
    },
    /// Retrieve all keys
    Getall,
}

#[derive(Args, Debug)]
pub struct Entry {
    pub key: String,
    pub value: String,
}
