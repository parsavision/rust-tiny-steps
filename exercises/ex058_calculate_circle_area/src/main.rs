fn circle_are(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}
fn main() {
    let radius = 5.0;
    let area = circle_are(radius);
    println!("The area of a circle with radius {} is {}", radius, area);
}
