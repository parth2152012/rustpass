use clap::{Parser, Subcommand};

use chrono::{DateTime, Local};

use std::error::Error;

// Declare modules
mod commands;
mod input;
mod models;
mod storage;

// Import module items
use models::Entry;

use storage::{load_entries, save_entries};

use input::{get_input, get_secure_input};

use commands::{delete_entry, list_entries, search_entries};

// --------------------------------------------------
// CLI parser
// --------------------------------------------------
#[derive(Parser)]
#[command(name = "RustPass")]
#[command(version = "1.0")]
#[command(about = "Simple Rust Password Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// --------------------------------------------------
// CLI command enum
// --------------------------------------------------
#[derive(Subcommand)]
enum Commands {
    Add,

    List,

    Search { service: String },

    Delete { service: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let file_path = "passwords.json";

    let mut entries = load_entries(file_path)?;

    match cli.command {
        Commands::Add => {
            let service = get_input("Enter Service name: ")?;

            let username = get_input("Enter Username: ")?;

            let password = get_secure_input("Enter Password: ")?;

            let current_time: DateTime<Local> = Local::now();

            let account_entry = Entry {
                service,
                username,
                password,
                created_at: current_time,
            };

            entries.push(account_entry);

            save_entries(file_path, &entries)?;

            println!("\nPassword saved successfully!");
        }

        Commands::List => {
            list_entries(&entries);
        }

        Commands::Search { service } => {
            search_entries(&entries, &service);
        }

        Commands::Delete { service } => {
            delete_entry(&mut entries, &service);

            save_entries(file_path, &entries)?;
        }
    }

    Ok(())
}
