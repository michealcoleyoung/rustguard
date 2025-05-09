use inquire::{Select, Text};

fn main() {
    println!("Welcome to RustGuard! \n");

    let options = vec![
        "Add a password",
        "View all passwords",
        "Delete a password",
        "Exit",
    ];

    let answer = Select::new("What would you like to do", options).prompt();

    match answer {
        Ok(choice) => match choice {
            "Add a password" => {
                add_password();
            }
            _ => println!("You selected {}", choice),
        },
        Err(_) => println!("There was an error reading your input."),
    }
}

fn add_password() {
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
}
