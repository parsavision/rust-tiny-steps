use std::io;

fn main() {
    loop {
        println!("type something: ");
        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("cannot read input");
        println!("You typed : {}", input_value);
    }
}
