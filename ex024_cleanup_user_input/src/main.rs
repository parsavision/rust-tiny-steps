use std::io;

fn main() {
    loop {
        println!("Type something and I'll echo it back to you : ");
        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line!");
        println!("You typed : {}", input_value.trim());
    }
}
