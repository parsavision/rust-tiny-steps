fn check_range(n: i32, low: i32, high: i32) -> bool {
    if n > low && n < high { true } else { false }
}
fn main() {
    let n = 5;
    let low = 1;
    let high = 10;
    println!(
        "Is {} in range [{}..{}]? {}",
        n,
        low,
        high,
        check_range(n, low, high)
    );
}
