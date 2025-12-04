fn grade_calculator(score: f32) -> &'static str {
    if score >= 90.0 {
        "A"
    } else if score >= 80.0 {
        "B"
    } else if score >= 70.0 {
        "C"
    } else if score >= 60.0 {
        "D"
    } else {
        "F"
    }
}

fn main() {
    let score = 85.0;
    let grade = grade_calculator(score);
    println!("Your grade is: {}", grade);
}
