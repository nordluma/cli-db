use clap::{Args, Parser, Subcommand};
use sqlx::FromRow;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(Entry),
    Get { key: String },
    Remove { key: String },
}

#[derive(Args, Debug, FromRow)]
pub struct Entry {
    pub key: String,
    pub value: String,
}
