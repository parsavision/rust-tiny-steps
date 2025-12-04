fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
fn main() {
    println!("The bigger number is {}", max(7, 10));
}
