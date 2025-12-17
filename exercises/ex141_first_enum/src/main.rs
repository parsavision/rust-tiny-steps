#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
fn main() {
    let player_move = Direction::Down;
    println!("Player moved {:?}", player_move);
}
