fn main() {
    let mut total_sum = 0;
    let start_number = 1;
    let end_number = 100;
    for current_number in start_number..=end_number {
        total_sum += current_number;
        if current_number == end_number {
            println!("Final sum: {}", total_sum);
        }
    }
}
