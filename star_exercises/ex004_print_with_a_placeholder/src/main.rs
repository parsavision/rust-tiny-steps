fn main() {
    let name_1 = "ali";
    let name_2 = String::from("mohammad");
    // i have a question why these two lines works same?
    println!("Hi {name_1} and {}", name_2);
    println!("Hi {name_1} and {}", &name_2);
}
