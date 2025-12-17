fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut even_numbers = Vec::new();
    for number in &numbers {
        if *number % 2 == 0 {
            even_numbers.push(number);
        }
    }
    println!("Numbers: {:?}", numbers);
    println!("Even numbers: {:?}", even_numbers);
}
