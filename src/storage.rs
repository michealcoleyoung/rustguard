use crate::models::PassWordEntry;
use ::std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

const VAULT_FILE: &str = "passwords.json";

pub fn load_passwords() -> Vec<PassWordEntry> {
    // Check if file exists
    if !Path::new(VAULT_FILE).exists() {
        return Vec::new();
    }

    // Read file contents
    let mut file = match File::open(VAULT_FILE) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Vec::new();
    }

    // Parse JSON
    match serde_json::from_str(&contents) {
        Ok(entries) => entries,
        Err(_) => Vec::new(),
    }
}

pub fn save_passwords(entries: &[PassWordEntry]) -> std::io::Result<()> {
    // Convert to JSON
    let json = serde_json::to_string_pretty(entries)?;

    // Write to file
    let mut file = File::create(VAULT_FILE)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
