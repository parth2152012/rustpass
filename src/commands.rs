use crate::models::Entry;

// --------------------------------------------------
// List all entries
// --------------------------------------------------
pub fn list_entries(entries: &Vec<Entry>) {
    if entries.is_empty() {
        println!("\nNo passwords stored.");
        return;
    }

    println!("\n--- Stored Passwords ---");

    for (index, entry) in entries.iter().enumerate() {
        let formatted_time = entry.created_at.format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "\n{}. Service: {}\n   Username: {}\n   Password: {}\n   Created At: {}",
            index + 1,
            entry.service,
            entry.username,
            entry.password,
            formatted_time
        );
    }
}

// --------------------------------------------------
// Search entries
// --------------------------------------------------
pub fn search_entries(entries: &Vec<Entry>, service_name: &str) {
    let mut found = false;

    for entry in entries {
        if entry
            .service
            .to_lowercase()
            .contains(&service_name.to_lowercase())
        {
            found = true;

            println!(
                "\nService: {}\nUsername: {}\nPassword: {}",
                entry.service, entry.username, entry.password
            );
        }
    }

    if !found {
        println!("\nNo matching service found.");
    }
}

// --------------------------------------------------
// Delete entries
// --------------------------------------------------
pub fn delete_entry(entries: &mut Vec<Entry>, service_name: &str) {
    let original_length = entries.len();

    entries.retain(|entry| entry.service.to_lowercase() != service_name.to_lowercase());

    if entries.len() < original_length {
        println!("\nEntry deleted successfully!");
    } else {
        println!("\nNo matching service found.");
    }
}
