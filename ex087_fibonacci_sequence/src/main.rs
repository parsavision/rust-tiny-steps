fn main() {
    // 0 1 1 2 3 5 8 13
    // 1 2 3 4 5 6 7 8
    let mut current = 0;
    let mut next = 1;
    let range = 10;

    for _ in 0..range {
        println!("{}", current);
        let temp = current + next;
        current = next;
        next = temp;
    }
}
