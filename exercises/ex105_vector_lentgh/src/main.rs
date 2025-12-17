fn main() {
    let mut names = vec!["mamad", "ali", "reza"];
    println!("{:?}, {:?}", names, names.len());
    names.pop();
    println!("{:?}, {:?}", names, names.len());
}
