fn main() {
    struct Users {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let first_user = Users {
        active: true,
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
    };
    println!("First user's email: {}", first_user.email);
    println!("First user's username : {}", first_user.username);
}
