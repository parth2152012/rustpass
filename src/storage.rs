use std::error::Error;
use std::fs;
use std::path::Path;

use crate::models::Entry;

// --------------------------------------------------
// Load entries from JSON
// --------------------------------------------------
pub fn load_entries(file_path: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path)?;

        if !data.trim().is_empty() {
            let entries: Vec<Entry> = serde_json::from_str(&data)?;

            return Ok(entries);
        }
    }

    Ok(Vec::new())
}

// --------------------------------------------------
// Save entries into JSON
// --------------------------------------------------
pub fn save_entries(file_path: &str, entries: &Vec<Entry>) -> Result<(), Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(entries)?;

    fs::write(file_path, json_data)?;

    Ok(())
}
