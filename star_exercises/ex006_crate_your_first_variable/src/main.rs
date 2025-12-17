fn main() {
    let _i8_number: i8 = 127;
    // 2^8 -1 = (16 * 16) - 1 = 160 + 60 + 36 -1 = 220 + 36 -1 = 256 -1 = 255
    // but because it signed, the max value is 127 and the min value is -128(consider zero!)
    // you can use _ before name variable to ignore clippy error for unused variable
    let _i8_number_min: i8 = -128;
    let _name = "mamad";
    // name = "ali"; this line will cause a compile error because name is immutable
    let mut _name = "mamad";
    _name = "ali"; //this line works because _name is mutable
    let mut _name_2 = String::from("mamad");
    println!("{}", _name_2);
    _name_2.push(' ');
    _name_2.push_str("ali");
    println!("{}", _name_2);
}
