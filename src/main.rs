use clap::Parser;
use cli_db::args::CliArgs;

#[tokio::main]
async fn main() {
    let cli = CliArgs::parse();

    match cli.commands {
        cli_db::args::Command::Add(entry) => {
            println!("Adding entry: {:?}", entry)
        }
        cli_db::args::Command::Get { key } => {
            println!("Retrieving value for: {}", key)
        }
        cli_db::args::Command::Remove { key } => {
            println!("Deleting entry: {}", key)
        }
    }
}
