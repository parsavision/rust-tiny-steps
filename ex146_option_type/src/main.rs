#[derive(Debug)]
struct Contact {
    name: String,
    phone: Option<String>,
}
fn main() {
    let alice = Contact {
        name: "Alice".to_string(),
        phone: Some("123-456-7890".to_string()),
    };
    let bob = Contact {
        name: "Bob".to_string(),
        phone: None,
    };
    println!("{:?}", alice);
    println!("{:?}", bob);
}
