use inquire::Select;
mod commands;

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
                commands::add_password();
            }
            _ => println!("You selected {}", choice),
        },
        Err(_) => println!("There was an error reading your input."),
    }
}
