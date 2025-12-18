fn main() {
    let _name = "mamad";
    //name = "ali"
    //This line won't work because name is immutable
    let mut name = "mamad";
    println!("{name}");
    name = "ali";
    println!("{name}");
}
