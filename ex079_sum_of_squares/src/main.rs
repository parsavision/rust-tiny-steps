fn main() {
    let mut sum = 0;
    let mut square = 0;
    for i in 1..=10 {
        square = i * i;
        sum = sum + square;
        if i == 10 {
            println!("the sum of square of 1 to {i} is {sum}");
        }
    }
}
