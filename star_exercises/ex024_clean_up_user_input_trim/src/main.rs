use std::io::{self, Write};

fn main() {
    let mut name = String::new();
    println!("Hello, Welcome my friend!");
    print!("What is your name? : ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Nice to meet you, {}!", name.trim());
}
