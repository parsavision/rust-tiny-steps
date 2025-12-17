use std::io;

fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    match input.trim().parse::<i32>() {
        Ok(number) => {
            let result = if number > 5 { "big" } else { "small" };
            println!("The result is: {result}");
        }
        Err(_) => {
            println!("invalid input");
        }
    }
}
