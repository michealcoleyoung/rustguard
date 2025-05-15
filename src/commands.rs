use inquire::Text;

pub fn add_password() {
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
