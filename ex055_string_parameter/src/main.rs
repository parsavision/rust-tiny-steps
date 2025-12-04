fn count_chars(s: &str) -> usize {
    s.len()
}
fn main() {
    let input = "Hello, world!";
    println!(
        "The string '{}' has {} characters.",
        input,
        count_chars(input)
    );
}
