use std::io;

fn main() {
    println!("Type a number: ");
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("failed to read line!");
}
