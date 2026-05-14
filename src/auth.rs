use serde::{Deserialize, Serialize};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use std::error::Error;
use std::fs;

use crate::input::get_secure_input;

// --------------------------------------------------
// Stores authentication configuration
//
// We ONLY store password hash
// NEVER the real password
// --------------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct Config {
    // Argon2 password hash string
    pub password_hash: String,
}

// --------------------------------------------------
// Hash master password using Argon2
//
// Returns:
// Result<String, Error>
//
// String = generated password hash
// --------------------------------------------------
fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    // Generate random cryptographic salt
    //
    // Salt prevents:
    // - rainbow table attacks
    // - identical hashes
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 hasher instance
    let argon2 = Argon2::default();

    // Hash password bytes + salt
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;

    // Convert hash object -> String
    Ok(password_hash.to_string())
}

// --------------------------------------------------
// Verify entered password against stored hash
//
// Returns:
// true  -> valid password
// false -> invalid password
// --------------------------------------------------
fn verify_password(hash: &str, password: &str) -> bool {
    // Parse stored hash string
    let parsed_hash = PasswordHash::new(hash);

    // Invalid hash format
    if parsed_hash.is_err() {
        return false;
    }

    let parsed_hash = parsed_hash.unwrap();

    // Verify password
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// --------------------------------------------------
// First-time setup
//
// Creates:
// config.json
//
// Stores:
// hashed master password
// --------------------------------------------------
pub fn setup_master_password() -> Result<(), Box<dyn Error>> {
    println!("\n=== First Time Setup ===");

    let password = get_secure_input("Create master password: ")?;

    let confirm_password = get_secure_input("Confirm master password: ")?;

    // Check password confirmation
    if password != confirm_password {
        println!("\nPasswords do not match.");

        return Ok(());
    }

    // Generate Argon2 hash
    let password_hash = hash_password(&password)?;

    // Create config struct
    let config = Config { password_hash };

    // Serialize config -> JSON
    let json = serde_json::to_string_pretty(&config)?;

    // Save config file
    fs::write("config.json", json)?;

    println!("\nMaster password created!");

    Ok(())
}

// --------------------------------------------------
// Login/authentication
//
// Returns:
// Ok(true)  -> login success
// Ok(false) -> invalid password
// --------------------------------------------------
pub fn login() -> Result<bool, Box<dyn Error>> {
    // Read config file
    let data = fs::read_to_string("config.json")?;

    // Deserialize JSON -> Config
    let config: Config = serde_json::from_str(&data)?;

    // Ask for master password
    let password = get_secure_input("Enter master password: ")?;

    // Verify entered password
    let valid = verify_password(&config.password_hash, &password);

    Ok(valid)
}
