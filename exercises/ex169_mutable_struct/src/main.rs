fn main() {
    struct Users {
        name: String,
        age: i32,
    }

    let mut user = Users {
        name: String::from("John Doe"),
        age: 30,
    };
    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    user.age = 31;
    println!("Updated Age: {}", user.age);
}
