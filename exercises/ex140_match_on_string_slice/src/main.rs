use std::io;

fn main() {
    println!("Hi! welcome to resturant!");
    println!("Choose the food you want from menu:");
    let mut input = String::new();
    loop {
        input.clear();
        println!("We have : \n1)pizza\n2)burger\n3)fries\n4)pasta");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "pizza" => {
                println!("You have chosen pizza");
                println!("Wait until your order is ready!");
                println!("It takes 10 minutes");
                break;
            }
            "burger" => {
                println!("You have chosen burger");
                println!("Wait until your order is ready!");
                println!("It takes 5 minutes");
                break;
            }
            "fries" => {
                println!("You have chosen fries");
                println!("Wait until your order is ready!");
                println!("It takes 3 minutes");
                break;
            }
            "pasta" => {
                println!("You have chosen pasta");
                println!("Wait until your order is ready!");
                println!("It takes 8 minutes");
                break;
            }
            _ => println!("Invalid choice, ensure you enter a right food name!"),
        };
    }
}
