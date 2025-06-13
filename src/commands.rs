use crate::models::PassWordEntry;
use crate::storage::{save_vault};
use crate::Vault;
use inquire::{Text, Confirm};

use std::error::Error;

// Type alias for results
type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Add a new password entry to vault
pub fn add_password(vault: &mut Vault, key: &[u8]) -> Result<()> {
   println!("Adding a new password entry:");

   let site = Text::new("Enter the site name:").prompt()?;
   let email = Text::new("Enter the email address:").prompt()?;
   let username = Text::new("Enter the username:").prompt()?;
   let password = Text::new("Enter the password:").prompt()?;

   let entry = PassWordEntry {
       site,
       email,
       username,
       password
   };

   vault.entries.push(entry);

   save_vault(vault, key)?;
   println!("Password added successfully.\n");

   Ok(())
}

// View all saved password entries
pub fn view_passwords(vault: &Vault) {
    if vault.entries.is_empty() {
        println!("You have no saved passwords");
        return;
    }

    println!("Saved password entries:");
    for (index, entry) in vault.entries.iter().enumerate() {
        println!(
            "{}. {} (Username: {})",
            index + 1,
            entry.site,
            entry.username
        );
    }

    // Prompt to select entry for more details
    let choice = match Text::new("Enter the number of an entry to view details (or press Enter to skip):")
        .prompt()
    {
        Ok(input) if input.is_empty() => return,
        Ok(input) => match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= vault.entries.len() => num - 1,
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

    let selected = &vault.entries[choice];

    let reveal = Confirm::new("Would you like to view the password?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    println!("\nSite: {}", selected.site);
    println!("Email: {}", selected.email);
    println!("Username: {}", selected.username);
    if reveal {
        println!("Password: {}", selected.password);
    } else {
        println!("Password: ********** (hidden)");
    }
    println!();
}

// Edit fields of an existing password entry
pub fn edit_password(vault: &mut Vault, key: &[u8]) -> Result<()> {
    if vault.entries.is_empty() {
        println!("No saved passwords to edit.");
        return Ok(());
    }

    println!("Select an entry to edit:");
    for (index, entry) in vault.entries.iter().enumerate() {
        println!(
            "{}. {} (Username: {})",
            index + 1,
            entry.site,
            entry.username
        );
    }

    let choice = match Text::new("Enter the number of the entry to edit:")
        .prompt()
    {
        Ok(input) => match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= vault.entries.len() => num - 1,
            _ => {
                println!("Invalid selection.");
                return Ok(());
            }
        },

        Err(_) => {
            println!("Error reading input.");
            return Ok(());
        }
    };

    let selected = &mut vault.entries[choice];

    macro_rules! update_field {
        ($prompt:expr, $field:expr) => {{
            let should_update = Confirm::new(&format!("Update {}?", $prompt))
                .with_default(false)
                .prompt()
                .unwrap_or(false);

            if should_update {
                let new_value = Text::new(&format!("Enter new {}: ", $prompt))
                    .prompt()
                    .ok();

                if let Some(value) = new_value {
                    $field = value;
                }
            }
        }};
    }
    
    // Invoke update_field macro
    update_field!("site", selected.site);
    update_field!("email", selected.email);
    update_field!("username", selected.username);
    update_field!("password", selected.password);

    save_vault(vault, key)?;
    println!("Entry updated successfully.\n");

    Ok(())
}

// Delete a password entry from the vault
pub fn delete_password(vault: &mut Vault, key: &[u8]) -> Result<()> {
    if vault.entries.is_empty() {
        println!("No saved passwords to delete.");
        return Ok(());
    }

    println!("Select an entry to delete:");
    for (index, entry) in vault.entries.iter().enumerate() {
        println!(
            "{}. {} (Username: {})",
            index + 1,
            entry.site,
            entry.username
        );
    }
    
    let choice = match Text::new("Enter the number of the entry to delete:")
        .prompt()
    {
        Ok(input) => match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= vault.entries.len() => num - 1,
            _ => {
                println!("Invalid selection.");
                return Ok(());
            }
        },
        Err(_) => {
            println!("Error reading input.");
            return OK(());
        }
    };

    let entry_to_delete = &vault.entries[choice];
    let confirm = Confirm::new("Are you sure you want to delete this entry?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    if confirm {
        vault.entries.remove(choice);
        save_vault(vault, key)?;
        println!("Entry deleted successfully.");
    } else {
        println!("Deletion canceled");
    }

    println!();

    Ok(())

}
