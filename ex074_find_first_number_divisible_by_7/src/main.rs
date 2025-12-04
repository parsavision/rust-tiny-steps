fn main() {
    for i in 1..=100 {
        if i % 7 == 0 {
            println!("{i}");
            break;
        }
    }
}
