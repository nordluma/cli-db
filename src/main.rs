use clap::Parser;
use cli_db::args::CliArgs;

fn main() {
    let cli = CliArgs::parse();

    match cli.commands {
        cli_db::args::Command::Add(entry) => {
            println!("Adding: {:?}", entry)
        }
        cli_db::args::Command::Remove(entry) => {
            println!("Removing: {:?}", entry)
        }
        cli_db::args::Command::Get(key) => {
            println!("Getting value for: {:?}", key)
        }
    }
}
