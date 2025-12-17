fn countdown(number: i32) {
    if number > 0 {
        println!("Countdown: {}", number);
        countdown(number - 1);
    }
}
fn main() {
    countdown(10);
}
