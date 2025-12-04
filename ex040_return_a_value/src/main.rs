fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    println!("{} + {} = {}", 1, 2, add(1, 2));
    println!("{} + {} = {}", 3, 4, add(3, 4));
    println!("{} + {} = {}", 223, 232, add(223, 232));
}
