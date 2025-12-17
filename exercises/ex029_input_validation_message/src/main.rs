use std::io;
fn main() {
    println!("Enter your number: ");
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line.");

    match input_value.trim().parse::<i32>() {
        Ok(num) => {
            let result = num * 2;
            println!("Result: {}", result);
        }
        Err(_) => {
            println!("Invalid input");
        }
    }
}
