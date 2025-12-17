fn main() {
    let mut names = vec!["Alice", "Bob", "Charlie"];
    println!("Names: {:?}", names);
    names.clear();
    println!("Names after clear: {:?}", names);
    println!("{}", names.is_empty());
}
