fn main() {
    let arr = [1, 3, 4, 5, 6, 2, 4, 55, 66, 23, 58];
    let mut temp = arr[0];
    for item in arr.iter() {
        if *item > temp {
            temp = *item;
        } else {
            continue;
        }
    }
    println!("The largest number is {}", temp);
}
