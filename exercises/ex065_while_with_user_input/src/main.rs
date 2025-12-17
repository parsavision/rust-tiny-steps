use std::io;

fn main() {
    println!("Please enter the password:");
    let password = String::from("123");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    while input.trim() != &password {
        println!("Incorrect password!, please try again.");
        println!("Please enter the password:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
    }
    println!("Correct password!");
}
