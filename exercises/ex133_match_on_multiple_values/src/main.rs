fn main() {
    let grade = 3.0;
    match grade {
        20.0 | 19.75 => println!("Excellent"),
        19.0..=19.75 => println!("Very Good"),
        16.0..=18.99 => println!("Good"),
        12.0..=15.99 => println!("Average"),
        10.0..=11.99 => println!("Below Average"),
        0.0..=10.0 => println!("What the heck are you doing?"),
        _ => println!("Invalid Grade"),
    }
}
