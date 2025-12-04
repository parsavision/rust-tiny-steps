fn main() {
    'outer: for i in 0..=5 {
        for _j in 0..i {
            print!("*");
            break 'outer;
        }
        println!();
    }
}
