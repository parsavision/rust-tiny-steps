fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7];
    for number in numbers.iter_mut() {
        *number *= 2;
    }
    println!("{:?}", numbers);
}
