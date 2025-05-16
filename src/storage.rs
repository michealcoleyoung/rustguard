use crate::models::{PassWordEntry, Vault};

pub fn load_passwords() -> Vec<PassWordEntry> {
    // Read file from logic
}

pub fn save_passwords(entries: &[PassWordEntry]) -> Result<(), std::io::Error> {
    // Write file to logic
}
