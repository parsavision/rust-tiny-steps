fn main() {
    let mut sum = 0;
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    for item in arr.iter() {
        sum = sum + item;
        index += 1;
        if index == arr.len() {
            println!("Final sum: {}", sum);
        }
    }
}
