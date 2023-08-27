use clap::Parser;

use cli_db::args::{CliArgs, Command, Entry};
use cli_db::store::Store;

#[tokio::main]
async fn main() {
    let store = Store::init().await.unwrap();
    let cli = CliArgs::parse();

    match cli.commands {
        Command::Add(entry) => {
            eprintln!("Adding entry: {:?}", entry);
            create_entry(store, entry).await;
        }
        Command::Get { key } => {
            println!("Retrieving value for: {}", key)
        }
        Command::Remove { key } => {
            println!("Deleting entry: {}", key)
        }
        Command::Getall => {
            get_all_entries(store).await;
        }
    }
}

async fn create_entry(mut store: Store, entry: Entry) {
    match store.insert(entry).await {
        Ok(_) => eprintln!("Entry added"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn get_all_entries(mut store: Store) {
    match store.get_all().await {
        Ok(entries) => {
            for entry in entries {
                println!("[{}]: {}", entry.id, entry.value)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
