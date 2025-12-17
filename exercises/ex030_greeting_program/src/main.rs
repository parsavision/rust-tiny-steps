use std::io;

fn main() {
    println!("Hi!!!! What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line.");
    println!("Hi {}!, Nice to meet you.", name.trim());
}
