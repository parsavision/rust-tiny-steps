use std::io;
fn main() {
    println!("Tell me your order: ");
    println!(
        "Here are your options: (give me the number of the food)\n1) Kebab\n2) Pizza\n3) Lasagna\n4) Pasta"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let food = match input.trim().parse::<u8>() {
        Ok(1) => "Kebab",
        Ok(2) => "Pizza",
        Ok(3) => "Lasagna",
        Ok(4) => "Pasta",
        _ => "Invalid input",
    };
    println!("You ordered {}!", food);
}
