fn resolve_status_code(option: Option<i32>) {
    match option {
        Some(code) => println!("Status code: {}", code),
        None => println!("No status code provided"),
    }
}

fn main() {
    resolve_status_code(Some(200));
    resolve_status_code(None);
}
