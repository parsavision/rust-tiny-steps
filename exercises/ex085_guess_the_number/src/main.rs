use rand::Rng;
use std::io;

fn main() {
    println!("GUESSING GAME");
    let random_number = rand::rng().random_range(1..=100);
    println!("Random number: {}", random_number);
    let mut chance = 5;
    loop {
        println!("Guess the number!");
        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");
        match input_value.trim().parse::<i32>() {
            Ok(num) => {
                if num > random_number {
                    println!("Too high!");
                    chance -= 1;
                    println!("Chances left: {}", chance);
                } else if num < random_number {
                    println!("Too low!");
                    chance -= 1;
                    println!("Chances left: {}", chance);
                } else {
                    println!("You guessed it!");
                    println!("Congratulations!");
                    println!("You won!");
                    break;
                }
                if chance == 0 {
                    println!("You lost!");
                    println!("The number was {}", random_number);
                    break;
                }
            }
            Err(_) => println!("Invalid input"),
        }
    }
}
