use crate::models::{PassWordEntry, Vault};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use serde::{Deserialize, Serialize};
use ::std::path::Path;
use std::fs;
use std::io::{self, Read, Write};

// Encrypted file where passwords are stored
const VAULT_FILE: &str = "vault.json";

// Type alias for results that may return any error type 
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Load and decrypt the vault using the provided key
pub fn load_vault(key: &[u8]) -> Result<Vault> {
    // Check if file exists
    if !Path::new(VAULT_FILE).exists() {
        return Ok(Vault {
            entries: Vec::new(),
            salt: generate_salt(),
        });
    }


    // Read file contents
    let mut file = fs::File::open(VAULT_FILE)?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = ChaCha20Poly1305::new(key.into());
    let decrypted = cipher.decrypt(nonce, ciphertext)?;
    let vault: Vault = serde_json::from_slice(&decrypted)?;
    Ok(vault)
}

// Encrypt and save the vault using the provided key
pub fn save_vault(vault: &Vault, key: &[u8]) -> Result<()> {
    let serialized = serde_json::to_vec(vault)?;
    let cipher = ChaCha20Poly1305::new(key.into());

    // Generate random nonce
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let nonce = Nonce::from_slice(&nonce);

    let encrypted = cipher.encrypt(nonce, &serialized[..])?;

    let mut output = Vec::with_capacity(nonce.len() + encrypted.len());
    output.extend_from_slice(nonce);
    output.extend_from_slice(&encrypted);

    // Write to file
    fs::write(VAULT_FILE, &output)?;
    Ok(())
}

fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    getrandom::getrandom(&mut salt).expect("Failed to generate random salt");
    salt
}
