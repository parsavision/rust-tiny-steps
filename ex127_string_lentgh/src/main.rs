fn main() {
    let s1 = String::from("Hello");
    println!("bytes: {}", s1.len());
    let s2 = String::from("سلام");
    println!("bytes: {}", s2.len());
    let s3 = String::from("你好");
    println!("bytes: {}", s3.len());
    let s4 = String::from("🦀");
    println!("bytes: {}", s4.len());
}
