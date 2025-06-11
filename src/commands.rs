use crate::models::PassWordEntry;
use crate::storage::{load_passwords, save_passwords};
use inquire::Text;
use std::usize;

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

    let email = match Text::new("Enter the email address:").prompt() {
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

// Allow user to edit individual details of password info
pub fn edit_password() {
    // Load entries
    let mut entries = load_passwords();

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
    let selected = &mut entries[choice];

    // Update site
    let update_site = inquire::Confirm::new("Update site")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    if update_site {
        let new_site = Text::new("Enter new site: ").prompt().ok();
        if let Some(site) = new_site {
            selected.site = site;
        }
    }
    // Update email
    let update_email = inquire::Confirm::new("Update email")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    if update_email {
        let new_email = Text::new("Enter new email: ").prompt().ok();
        if let Some(email) = new_email {
            selected.email = email;
        }
    }
    // Update email
    let update_username = inquire::Confirm::new("Update username")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    if update_username {
        let new_username = Text::new("Enter new username: ").prompt().ok();
        if let Some(username) = new_username {
            selected.username = username;
        }
    }
    // Update password
    let update_password = inquire::Confirm::new("Update password")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    if update_password {
        let new_password = Text::new("Enter new password: ").prompt().ok();
        if let Some(password) = new_password {
            selected.password = password;
        }
    }
    // Save passwords
    match save_passwords(&entries) {
        Ok(_) => println!("Password entry updated successfully!"),
        Err(e) => println!("Error saving changes: {}", e),
    }
}

pub fn delete_password() {
    // Load passwords
    let mut entries = load_passwords();
   
     // Check if empty
    if entries.is_empty() {
        println!("You have no saved passwords.");
        return;
    }

    // Display the password entries
    println!("Saved password entries: ");
    for (index, entry) in entries.iter().enumerate() {
        println!(
            "{}. {} (Username: {})",
            index + 1,
            entry.site,
            entry.username
        );
    }
    
    // Prompt the user to select an entry
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

    // Ask for confirmation before deleting
    let confirm = inquire::Confirm::new("Are you sure you want to delete this entry?")
        .with_default(false)
        .prompt();
    if !confirm.unwrap_or(false) {
        println!("Deletion cancelled.");
        return;
    }
    
    // Remove entry from vector
    entries.remove(choice);

    // Save updated list using save_passwords
      match save_passwords(&entries) {
        Ok(_) => println!("Password entry updated successfully!"),
        Err(e) => println!("Error saving changes: {}", e),
    }
}
