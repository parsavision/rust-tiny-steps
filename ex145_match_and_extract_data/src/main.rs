enum TodoCommand {
    Add { title: String, priority: u8 },
    Complete { id: u32 },
    List,
    Delete { id: usize },
    Quit,
}
fn main() {
    fn procces_commands(command: TodoCommand) {
        match command {
            TodoCommand::Add { title, priority } => {
                println!("Adding task: {} with priority {}", title, priority);
            }
            TodoCommand::Complete { id } => {
                println!("Completing task with id {}", id);
            }
            TodoCommand::List => {
                println!("Listing all tasks");
            }
            TodoCommand::Delete { id } => {
                println!("Deleting task with id {}", id);
            }
            TodoCommand::Quit => {
                println!("Quitting");
            }
        }
    }
    procces_commands(TodoCommand::Add {
        title: String::from("Buy milk"),
        priority: 2,
    });
    procces_commands(TodoCommand::Complete { id: 1 });
    procces_commands(TodoCommand::List);
    procces_commands(TodoCommand::Delete { id: 2 });
    procces_commands(TodoCommand::Quit);
}
