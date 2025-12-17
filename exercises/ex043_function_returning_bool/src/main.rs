fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        println!("{} is even", n);
        true
    } else {
        println!("{} is odd", n);
        false
    }
}
fn main() {
    is_even(4);
    is_even(5);
}
