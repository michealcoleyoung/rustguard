use crate::models::PassWordEntry;
use crate::storage::{load_passwords, save_passwords};
use inquire::Text;
use std::usize;
// use serde_json::Value;

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

pub fn view_passwords() {
    // Load entries
    let entries = load_passwords();

    // Check if empty
    if entries.is_empty() {
        println!("You have no saved passwords.");
        return;
    }
    println!("Saved password entries: ");
    for (index, entry) in entries.iter().enumerate() {
        println!(
            "{}. {} (Username: {})",
            index + 1,
            entry.site,
            entry.username
        );
    }
    // Prompt user to select an entry
    let choice = match Text::new(
        "Enter the number of the entry to view details (or press Enter to cancel):",
    )
    .prompt()
    {
        Ok(input) if input.is_empty() => return,
        Ok(input) => match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= entries.len() => num - 1,
            _ => {
                println!("Invalid selection.");
                return;
            }
        },
        Err(_) => {
            println!("Error reading input.");
            return;
        }
    };
    let selected = &entries[choice];

    // Ask user if they would like to view the password
    let reveal = match inquire::Confirm::new("Would you like to view the password")
        .with_default(false)
        .prompt()
    {
        Ok(answer) => answer,
        Err(_) => {
            println!("Error reading response.");
            return;
        }
    };
    println!("Site: {}", selected.site);
    println!("Email: {}", selected.email);
    println!("Username: {}", selected.username);
    if reveal {
        println!("Password: {}", selected.password);
    } else {
        println!("Password: ********** (hidden)");
    }
}

// pub fn edit_password() {

// }

// pub fn delete_passwords() {
//     // Load passwords
//     // Let user select password to delete
//     // Save updated list
// }
