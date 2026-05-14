use clap::{Parser, Subcommand};

use chrono::{DateTime, Local};

use std::error::Error;
use std::path::Path;

// --------------------------------------------------
// Declare project modules
// --------------------------------------------------
mod auth;
mod commands;
mod crypto;
mod input;
mod models;
mod storage;

// --------------------------------------------------
// Import auth functions
// --------------------------------------------------
use auth::{login, setup_master_password};

// --------------------------------------------------
// Import Entry struct
// --------------------------------------------------
use models::Entry;

// --------------------------------------------------
// Import storage functions
// --------------------------------------------------
use storage::{load_entries, save_entries};

// --------------------------------------------------
// Import input functions
// --------------------------------------------------
use input::{get_input, get_secure_input};

// --------------------------------------------------
// Import command functions
// --------------------------------------------------
use commands::{delete_entry, list_entries, search_entries};

// --------------------------------------------------
// CLI parser configuration
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
    // Add new password entry
    Add,

    // List all entries
    List,

    // Search entries
    Search { service: String },

    // Delete entry
    Delete { service: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    // --------------------------------------------------
    // FIRST-TIME SETUP
    // --------------------------------------------------

    // Check if config file exists
    if !Path::new("config.json").exists() {
        setup_master_password()?;
    }

    // --------------------------------------------------
    // LOGIN
    // --------------------------------------------------

    let authenticated = login()?;

    // Invalid password
    if !authenticated {
        println!("\nInvalid master password.");

        return Ok(());
    }

    println!("\nVault unlocked!");

    // --------------------------------------------------
    // PARSE CLI ARGUMENTS
    // --------------------------------------------------

    let cli = Cli::parse();

    // JSON database file
    let file_path = "passwords.json";

    // Load entries into memory
    let mut entries = load_entries(file_path)?;

    // --------------------------------------------------
    // MATCH COMMANDS
    // --------------------------------------------------

    match cli.command {
        // --------------------------------------------------
        // ADD COMMAND
        // --------------------------------------------------
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

            // Move entry into vector
            entries.push(account_entry);

            // Save updated vector
            save_entries(file_path, &entries)?;

            println!("\nPassword saved successfully!");
        }

        // --------------------------------------------------
        // LIST COMMAND
        // --------------------------------------------------
        Commands::List => {
            list_entries(&entries);
        }

        // --------------------------------------------------
        // SEARCH COMMAND
        // --------------------------------------------------
        Commands::Search { service } => {
            search_entries(&entries, &service);
        }

        // --------------------------------------------------
        // DELETE COMMAND
        // --------------------------------------------------
        Commands::Delete { service } => {
            delete_entry(&mut entries, &service);

            save_entries(file_path, &entries)?;
        }
    }

    Ok(())
}
