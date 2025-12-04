fn abs(number: i32) -> i32 {
    if number < 0 { -number } else { number }
}
fn main() {
    println!("The absolute value of number is : {}", abs(-5));
}
