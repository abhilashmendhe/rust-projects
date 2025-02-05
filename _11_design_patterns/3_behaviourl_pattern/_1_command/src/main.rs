/* 
Example taken from this website
https://www.hackingwithrust.net/2023/04/23/design-patterns-in-rust-the-command-a-simple-implementation-of-a-versatile-pattern/
*/

trait Command {
    fn execute(&self);
}

struct AddCommand {
    message: String
}

impl AddCommand {
    fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl Command for AddCommand {
    fn execute(&self) {
        println!("Executing: {}", self.message);
    }
}

struct SubCommand {
    message: String
}

impl SubCommand {
    fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl Command for SubCommand {
    fn execute(&self) {
        println!("Executing: {}", self.message);
    }
}

struct Invoker {
    commands: Vec<Box<dyn Command>>
}

impl Invoker {
    fn new() -> Invoker {
        Invoker {
            commands: Vec::new()
        }
    }

    fn append_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    fn execute_commands(&self) {
        for command in self.commands.iter() {
            command.execute();
        }
    }
}

fn main() {

    let mut invoker = Invoker::new();
    invoker.append_command(Box::new(AddCommand::new("Add Command".to_string())));
    invoker.append_command(Box::new(SubCommand::new("Sub Command".to_string())));

    invoker.execute_commands();
}
