use std;
use std::io;
use std::io::Write;
use vm::VM;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                },
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
