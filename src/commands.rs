use crate::models::PassWordEntry;
use crate::storage::{load_passwords, save_passwords};
use inquire::Text;

pub fn add_password() {
    // Collect input from user
    // Create PasswordEntry
    // Call storage functions to save

    let site = match Text::new("Enter the site name:").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading site name");
            return;
        }
    };

    let email = match Text::new("Enter the email addresss:").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading email address");
            return;
        }
    };

    let username = match Text::new("Enter the username:").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading username");
            return;
        }
    };

    let password = match Text::new("Enter the password:").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Error reading password");
            return;
        }
    };

    println!("The account information you've provided is listed below:");
    println!("site: {}", site);
    println!("email: {}", email);
    println!("username: {}", username);
    println!("password: {}", password);

    let entry = PassWordEntry {
        site,
        email,
        username,
        password,
    };

    // Load existing passwords
    let mut entries = load_passwords();

    // Add new entry
    entries.push(entry);

    // Save all entries
    match save_passwords(&entries) {
        Ok(_) => println!("Password saved successfully!"),
        Err(e) => println!("Error saving password: {}", e),
    }
}

// pub fn view_passwords() {
//     // Load passwords
//     // Display to user
//     {}
// }

// pub fn delete_passwords() {
//     // Load passwords
//     // Let user select password to delete
//     // Save updated list
// }
