fn main() {
    let s1 = String::from("Hello");
    println!("{s1}");
    println!("bytes: {}", s1.len());
    println!("characters: {}", s1.chars().count());
    let s2 = String::from("سلام");
    println!("{s2}");
    println!("bytes: {}", s2.len());
    println!("characters: {}", s2.chars().count());
    let s3 = String::from("你好");
    println!("{s3}");
    println!("bytes: {}", s3.len());
    println!("characters: {}", s3.chars().count());
    let s4 = String::from("🦀");
    println!("{s4}");
    println!("bytes: {}", s4.len());
    println!("characters: {}", s4.chars().count());
}
