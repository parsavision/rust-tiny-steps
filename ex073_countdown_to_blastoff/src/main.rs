fn main() {
    for i in (1..=10).rev() {
        println!("{}", i);
        if i == 1 {
            println!("Blast off!");
        }
    }
}
