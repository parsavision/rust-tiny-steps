#[derive(Debug)]
enum Command {
    Undo,
    Redo,
    Copy,
    Paste,
    Exit,
}

fn execute(command: Command) {
    match command {
        Command::Undo => println!("Undoing"),
        Command::Redo => println!("Redoing"),
        Command::Copy => println!("Copying"),
        Command::Paste => println!("Pasting"),
        Command::Exit => println!("Exiting"),
    }
}

fn main() {
    let command = Command::Copy;
    execute(command);
    let command = Command::Paste;
    execute(command);
}
