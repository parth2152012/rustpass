use crate::crypto::{decrypt_data, encrypt_data};

use crate::models::Entry;

use std::error::Error;
use std::fs;
use std::path::Path;

// --------------------------------------------------
// Load encrypted vault
//
// Flow:
// vault.enc
// -> decrypt
// -> JSON
// -> Vec<Entry>
// --------------------------------------------------
pub fn load_entries(file_path: &str, key: &[u8; 32]) -> Result<Vec<Entry>, Box<dyn Error>> {
    // First launch:
    // vault does not exist yet
    if !Path::new(file_path).exists() {
        return Ok(Vec::new());
    }

    // Read encrypted file
    let encrypted_data = fs::read_to_string(file_path)?;

    // Empty vault file
    if encrypted_data.trim().is_empty() {
        return Ok(Vec::new());
    }

    // AES decrypt
    let decrypted_json = decrypt_data(&encrypted_data, key)?;

    // Deserialize JSON
    let entries: Vec<Entry> = serde_json::from_str(&decrypted_json)?;

    Ok(entries)
}

// --------------------------------------------------
// Save encrypted vault
//
// Flow:
// Vec<Entry>
// -> JSON
// -> encrypt
// -> vault.enc
// --------------------------------------------------
pub fn save_entries(
    file_path: &str,
    entries: &Vec<Entry>,
    key: &[u8; 32],
) -> Result<(), Box<dyn Error>> {
    // Serialize entries -> JSON
    let json_data = serde_json::to_string_pretty(entries)?;

    // AES encrypt JSON
    let encrypted_data = encrypt_data(&json_data, key)?;

    // Save encrypted data
    fs::write(file_path, encrypted_data)?;

    Ok(())
}
