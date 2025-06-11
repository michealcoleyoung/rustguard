use inquire::Select;
mod commands;
mod models;
mod storage;
mod utils;

use crate::models::Vault;
use crate::storage::{load_vault, save_vault};
use crate::utils::derive_key;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to RustGuard! \n");
    
    // Prompt for master password
    let master_password = inquire::Password::new("Enter master password:")
        .with_show_mask(true)
        .prompt()?;

    // Load existing vault or create new one
    let mut vault = match load_vault(master_password.as_bytes()) {
        Ok(v) => v, 
        Err(e) => {
            println!("Creating new vault...");
            Vault {
                entries: Vec::new(),
                salt: [0u8; 16], // Generate fresh salt
            }
        }
    };

    // Derive encryption key from password + salt
    let key = derive_key(&master_password, &vault.salt);

    // Load interactive CLI menu
    let options = vec![
        "Add a password",
        "View all passwords",
        "Edit password",
        "Delete a password",
        "Exit",
    ];
    
    loop {
    let answer = Select::new("What would you like to do", options).prompt();

    match answer {
        Ok(choice) => match choice {
            "Add a password" => {
                if let Err(e) = commands::add_password(&mut vault, &key) {
                    eprintln!("Error adding password: {}", e);
                }
                
                if let Err(e) = save_vault(&vault, &key) {
                    eprintln!("Error saving vault: {}", e);
                }
            }
            "View all passwords" => {
                commands::view_passwords(&vault);
            }
            "Edit password" => {
                if let Err(e) = commands::edit_password(&mut vault, &key) {
                    eprintln!("Error editing password: {}", e);
                }
                if let Err(e) = save_vault(&vault, &key) {
                    eprintln!("Error saving vault: {}", e);
                }
            }
            "Delete a password" => {
                if let Err(e) = commands::delete_password(&mut vault, &key) {
                    eprintln!("Error deleting password: {}", e);
                }
                if let Err(e) = save_vault(&vault, &key) {
                    eprintln!("Error saving vault: {}", e);
                }
            }
            "Exit" => break,
            _ => {}
        },
        Err(_) => println!("There was an error reading your input."),
    
        }
    }
    println!("Goodbye!");
    Ok(())
        
}
