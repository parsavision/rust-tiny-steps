fn main() {
    let numbers = [23, 45, 66, 42, 2, 35, 10, 1, 99, 71, 100];
    let mut min = numbers[0];
    for number in numbers.iter() {
        if *number < min {
            min = *number;
        } else {
            continue;
        }
    }
    println!("The minimum number is {}", min);
}
