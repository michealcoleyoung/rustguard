pub(crate) use inquire::Select;
mod commands;
mod models;
mod storage;

fn main() {
    println!("Welcome to RustGuard! \n");

    let options = vec![
        "Add a password",
        "View all passwords",
        "Edit password",
        "Delete a password",
        "Exit",
    ];

    let answer = Select::new("What would you like to do", options).prompt();

    match answer {
        Ok(choice) => match choice {
            "Add a password" => {
                commands::add_password();
            }
            "View all passwords" => {
                commands::view_passwords();
            }
            _ => println!("You selected {}", choice),
        },
        Err(_) => println!("There was an error reading your input."),
    }
}
