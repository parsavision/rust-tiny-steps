use std::io;

fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    match input.trim().parse::<i32>() {
        Ok(number) => {
            if number > 100 {
                println!("The number is greater than 100");
            } else if number == 100 {
                println!("The number is equal to 100");
            } else {
                println!("Your number is lower than 100")
            }
        }
        Err(_) => {
            println!("invalid input");
        }
    }
}
