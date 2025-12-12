fn main() {
    let person = (26, "mamad", false);
    match person {
        (age @ 18..=25, name, true) => println!("{} is {} years old and is a student", name, age),
        (age @ 26..=30, name, false) => println!("{} is {} years old and is a worker", name, age),
        _ => println!("Unknown person"),
    }
}
