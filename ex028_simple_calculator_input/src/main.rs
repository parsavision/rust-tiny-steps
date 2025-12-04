use std::io;

fn main() {
    println!("Enter your first number: ");
    let mut input_1 = String::new();
    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read line");
    println!("Enter your second number: ");
    let number_1 = input_1.trim().parse::<i32>().expect("Invalid input");
    let mut input_2 = String::new();
    io::stdin()
        .read_line(&mut input_2)
        .expect("failed to read line");
    let number_2 = input_2.trim().parse::<i32>().expect("Invalid input");
    println!(
        "The addition of first number and second number is: {}",
        number_1 + number_2
    );
}
