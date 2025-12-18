use std::io::{self, Write};
fn main() {
    print!("Please Enter your name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut name = String::new();
    let input = io::stdin().read_line(&mut name);
    match input {
        Ok(_) => println!("Hello, {}!", name.trim()),
        Err(_) => println!("Error: failed to read input"),
    };
}
