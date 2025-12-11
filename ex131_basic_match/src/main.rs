use std::io;

fn main() {
    println!("Enter the number of month to display: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    match number.trim().parse::<u8>() {
        Ok(1) => println!("January"),
        Ok(2) => println!("February"),
        Ok(3) => println!("March"),
        Ok(4) => println!("April"),
        Ok(5) => println!("May"),
        Ok(6) => println!("June"),
        Ok(7) => println!("July"),
        Ok(8) => println!("August"),
        Ok(9) => println!("September"),
        Ok(10) => println!("October"),
        Ok(11) => println!("November"),
        Ok(12) => println!("December"),
        _ => println!("Invalid month number"),
    }
}
