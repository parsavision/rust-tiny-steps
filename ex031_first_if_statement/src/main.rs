use std::io;

fn main() {
    println!("Enter a number : ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");
    match number.trim().parse::<i32>() {
        Ok(num) => {
            if num > 10 {
                println!("You enter a big number!");
            } else {
                println!("You enter a small number!");
            }
        }
        Err(_) => {
            println!("You enter a invalid input.");
        }
    }
}
