use std::io;

fn main() {
    println!("Type a number that you want to double: ");
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).unwrap();
    let number = input_value.trim().parse::<i32>().unwrap();
    println!("The output is: {}", number * 2);
}
