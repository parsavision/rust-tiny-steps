fn check_positive(n: i32) -> bool {
    if n > 0 { true } else { false }
}
fn main() {
    let n = 5;
    println!("Is {} positive? {}", n, check_positive(n));
}
