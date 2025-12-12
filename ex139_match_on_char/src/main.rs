fn main() {
    let character = 'A';
    match character {
        c if c.is_uppercase() => println!("The character {c} is uppercase"),
        c if c.is_lowercase() => println!("The character {c} is lowercase"),
        c if c.is_numeric() => println!("The character {c} is numeric"),
        _ => println!("The character is not a letter or number"),
    }
}
