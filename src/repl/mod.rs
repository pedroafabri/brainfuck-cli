use std::io::{self, Write};

use crate::interpreter::BrainfuckInterpreter;

#[derive(PartialEq)]
enum CommandResult {
    Continue,
    Exit,
}

pub struct ReplSession {
    interpreter: BrainfuckInterpreter,
}

impl ReplSession {
    pub fn new() -> ReplSession {
        ReplSession {
            interpreter: BrainfuckInterpreter::new(),
        }
    }

    pub fn start(&mut self) {
        println!("Brainfuck REPL - v{}", env!("CARGO_PKG_VERSION"));
        println!("Type \".help\" for more information.");
        self.repl_loop();
    }

    fn repl_loop(&mut self) {
        let mut buffer = String::from("");

        loop {
            print!("bf> ");
            io::stdout().flush().unwrap();
            buffer.clear();

            if io::stdin().read_line(&mut buffer).is_ok() {
                buffer = buffer.trim().to_string();
                let keep_running = self.process_command(&buffer);
                if keep_running == CommandResult::Exit {
                    break;
                }
            }
            println!("");
        }
    }

    fn process_command(&mut self, cmd: &str) -> CommandResult {
        match cmd {
            ".exit" => { return CommandResult::Exit; }
            ".reset" => { self.reset_state(); }
            ".help" => { self.show_help(); }
            _ => { self.interpreter.execute(&cmd.to_string()).unwrap(); }
        };
        CommandResult::Continue
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!(".help  -> Display this message");
        println!(".reset -> Resets the current Brainfuck interpreter to its initial state");
        println!(".exit  -> Exits this REPL session");
    }

    fn reset_state(&mut self) {
        self.interpreter.reset();
        println!("Interpreter state reset.");
    }
    
}
