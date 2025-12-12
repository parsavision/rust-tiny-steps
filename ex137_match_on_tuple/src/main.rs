fn main() {
    let status = (200, "OK");
    match status {
        (200, "OK") => println!("Success"),
        (404, _) => println!("Not Found"),
        (500, _) => println!("Internal Server Error"),
        _ => println!("Unknown Status"),
    }
}
