fn main() {
    let mut counter = 0;
    loop {
        println!("Hello");
        counter += 1;
        if counter == 5 {
            break;
        }
    }
}
