fn main() {
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace"];
    for (index, name) in names.iter().enumerate() {
        println!("{} is at index {}", name, index);
    }
}
