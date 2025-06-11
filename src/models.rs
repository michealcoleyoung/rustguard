use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassWordEntry {
    pub site: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

// Holds all password entries, salt for key derivation hashed master password
#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<PassWordEntry>,
    pub salt: [u8; 16],
}
