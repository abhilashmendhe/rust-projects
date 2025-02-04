trait OS {
    fn execute(&self, cmd: String);
}

enum OSType {
    LINUX, 
    WINDOWS,
    MACOS
}

struct Linux;
struct Windows;
struct MacOS;

impl OS for Linux {
    fn execute(&self, cmd: String) {
        println!("'{}' command executed on linux.", cmd);
    }
}

impl OS for Windows {
    fn execute(&self, cmd: String) {
        println!("'{}' command executed on windows.", cmd);
    }
}

impl OS for MacOS {
    fn execute(&self, cmd: String) {
        println!("'{}' command executed on mac os.", cmd);
    }
}

struct OSFactory;

impl OSFactory {
    fn execute_command(os: &OSType) -> Box<dyn OS> {
        match os {
            OSType::LINUX => Box::new(Linux), 
            OSType::WINDOWS => Box::new(Windows),
            OSType::MACOS => Box::new(MacOS)
        }
    }  
}

fn main() {

    OSFactory::execute_command(&OSType::LINUX).execute("ls".to_string());
    OSFactory::execute_command(&OSType::WINDOWS).execute("ls".to_string());
    OSFactory::execute_command(&OSType::MACOS).execute("ls".to_string());
    // os.execute("ls".to_string());
}
