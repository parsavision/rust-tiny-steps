fn main() {
    let mut names = vec!["Alice", "Bob", "Charlie"];
    names.insert(1, "Dave");
    //names.insert(12344, "test"); it panics!!!
    println!("Names: {:?}", names);
}
