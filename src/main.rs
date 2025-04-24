use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter something");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered: {}", input);
}
