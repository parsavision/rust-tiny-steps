enum TrafficLight {
    Red,
    Yellow,
    Green,
}
fn main() {
    let current_light = TrafficLight::Red;
    match current_light {
        TrafficLight::Green => println!("Go"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Red => println!("Stop"),
    }
}
