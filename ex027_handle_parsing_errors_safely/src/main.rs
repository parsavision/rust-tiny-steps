use std::io;

fn main() {
    println!("Type a number that you want to double: ");
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");
    let number = input_value.trim().parse::<i32>().expect("Failed to parse");
    println!("The output is: {}", number * 2);
}
