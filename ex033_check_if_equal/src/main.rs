use std::io;

fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    match input.trim().parse::<i32>() {
        Ok(number) => {
            if number == 42 {
                println!("The answer!");
            } else {
                println!("not the answer.");
            }
        }
        Err(_) => {
            println!("Invalid input");
        }
    }
}
