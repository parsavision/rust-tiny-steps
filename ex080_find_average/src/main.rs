fn main() {
    let mut sum = 0;
    let mut counter = 0;
    for i in 0..=100 {
        sum += i;
        counter += 1;
    }
    println!("the avarage is {}", sum as f64 / counter as f64);
}
