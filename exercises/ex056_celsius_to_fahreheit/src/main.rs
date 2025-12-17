fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    1.8 * celsius + 32.0
}

fn main() {
    let temperature = 25.0;
    let fahrenheit = celsius_to_fahrenheit(temperature);
    println!("{}°C is {}°F", temperature, fahrenheit);
}
