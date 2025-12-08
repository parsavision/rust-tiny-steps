fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    match names.get(199999) {
        Some(name) => println!("Name: {}", name),
        None => println!("No name found"),
    }
}
