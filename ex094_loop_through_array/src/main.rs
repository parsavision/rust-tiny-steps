fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut index = 1;
    for item in arr.iter() {
        println!("The item {index} of array is {}", item);
        index += 1;
    }
}
