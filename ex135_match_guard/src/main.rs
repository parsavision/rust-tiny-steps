use std::io;

fn main() {
    println!("Enter your number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input {
        ref n if n.trim().parse::<i32>().unwrap() % 2 == 0 => println!("It is even"),
        _ => println!("It is odd"),
    }
}
