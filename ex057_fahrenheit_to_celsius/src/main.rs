fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let fahrenheit = 100.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);
}
