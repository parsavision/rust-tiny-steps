use std::io;
fn main() {
    println!("Enter a number to find factorial:");
    let mut sum = 1;
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    for n in 1..=number.trim().parse::<i32>().expect("Please type a number!") {
        sum = sum * n;
        if n == number.trim().parse::<i32>().expect("Please type a number!") {
            println!("Final sum: {}", sum);
        }
    }
}
