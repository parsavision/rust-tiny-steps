fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: f64, b: f64) -> f64 {
    a / b
}
fn main() {
    println!("The sum of {} and {} is : {}", 2, 3, add(2, 3));
    println!("The difference of {} and {} is : {}", 2, 3, subtract(2, 3));
    println!("The product of {} and {} is : {}", 2, 3, multiply(2, 3));
    println!("The quotient of {} and {} is : {}", 2, 3, divide(2.0, 3.0));
}
