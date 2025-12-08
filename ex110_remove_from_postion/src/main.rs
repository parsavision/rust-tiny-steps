fn main() {
    let mut names = vec!["Alice", "Bob", "Charlie"];
    println!("Names: {:?}", names);
    names.remove(1);
    println!("Names after removal: {:?}", names);
}
