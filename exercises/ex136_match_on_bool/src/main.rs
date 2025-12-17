use rand::Rng;

fn main() {
    let random_number_1 = rand::rng().random_range(50..=100);
    let random_number_2 = rand::rng().random_range(1..=50);
    println!("Random Number 1: {}", random_number_1);
    println!("Random Number 2: {}", random_number_2);
    println!("HI! Welcome to calculation game!");
    println!("What is the answer of {random_number_1} / {random_number_2} ? ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let answer = random_number_1 / random_number_2;
    match input.trim().parse::<i32>() {
        Ok(user_answer) => match user_answer == answer {
            true => println!("Correct!"),
            false => println!("Incorrect!"),
        },
        Err(_) => println!("Invalid input!"),
    }
}
