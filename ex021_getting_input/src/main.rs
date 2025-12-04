use std::io;

fn main() {
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value);
    println!("You typed {input_value}");
}
