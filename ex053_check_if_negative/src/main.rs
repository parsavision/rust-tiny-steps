fn check_negative(n: i32) -> bool {
    if n < 0 { true } else { false }
}
fn main() {
    let number = 5;
    let is_negative = check_negative(number);
    println!("Is {} negative? {}", number, is_negative);
}
