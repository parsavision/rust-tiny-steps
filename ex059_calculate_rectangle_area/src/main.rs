fn rectangle_area(height: f64, width: f64) -> f64 {
    (height * width) / 2.0
}

fn main() {
    let area = rectangle_area(10.0, 5.0);
    println!("The area of the rectangle is {}", area);
}
