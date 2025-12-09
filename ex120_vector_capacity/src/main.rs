fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("vector's initial contents: {:?}", numbers);
    println!("vector's capacity: {}", numbers.capacity());
    println!("vector's len: {}", numbers.len());
    numbers.push(6);
    println!("vector's contents after push: {:?}", numbers);
    println!("vector's len: {}", numbers.len());
    println!("vector's capacity: {}", numbers.capacity());
}
