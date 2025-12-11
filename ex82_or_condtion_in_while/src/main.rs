fn main() {
    let grade = 16;
    while grade < 0 || grade > 20 {
        println!("your grade({grade}) is invalid");
    }
    println!("your grade({grade}) is valid");
}
