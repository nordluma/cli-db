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
            eprintln!("Retrieving value for: {}", key);
            get_entry(store, &key).await;
        }
        Command::Remove { key } => {
            println!("Deleting entry: {}", key);
            delete_entry(store, &key).await;
        }
        Command::Getall => {
            get_all_entries(store).await;
        }
        Command::Update(entry) => {
            update_entry(store, entry).await;
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

async fn get_entry(mut store: Store, key: &str) {
    match store.get_entry(key).await {
        Ok(opt_entry) => match opt_entry {
            Some(entry) => println!("[{}]: {}", entry.id, entry.value),
            None => eprintln!("No entry found"),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn update_entry(mut store: Store, entry: Entry) {
    match store.update(entry).await {
        Ok(entry) => {
            eprintln!("Value updated");
            println!("[{}]: {}", entry.id, entry.value);
        }
        Err(e) => eprintln!("Could not update entry: {}", e),
    }
}

async fn delete_entry(mut store: Store, key: &str) {
    match store.delete(key).await {
        Ok(_) => eprintln!("Entry deleted"),
        Err(e) => eprintln!("Could not delete entry: {}", e),
    }
}
