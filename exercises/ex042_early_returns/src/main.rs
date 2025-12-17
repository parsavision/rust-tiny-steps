fn check_number(n: i32) -> i32 {
    if n < 0 {
        println!("Negative");
        return 0;
    }

    println!("Positive");
    return 1;
}
fn main() {
    let result_1 = check_number(5);
    println!("Result: {}", result_1);
    let result_2 = check_number(-3);
    println!("Result: {}", result_2);
}
