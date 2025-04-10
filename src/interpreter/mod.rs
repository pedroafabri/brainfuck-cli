mod errors;

use std::{collections::HashMap, io::{Read, Write}};
use errors::InterpreterError;
use std::io;

pub struct BrainfuckInterpreter {
    memory: [u8; 30_000],
    memory_index: usize,
    loops: HashMap<usize, usize>,
    pc: usize
}

impl BrainfuckInterpreter {
    pub fn new() -> BrainfuckInterpreter {
        BrainfuckInterpreter { 
            memory: [0u8; 30_000],
            memory_index: 0,
            loops: HashMap::new(),
            pc: 0
        }
    }

    pub fn execute(&mut self, code: &String) -> Result<(), InterpreterError> {
        self.execution_reset();
        let operands: Vec<char> = code.chars().collect();
        self.map_loops(&operands)?;
        self.run_code(&operands);
        Ok(())
    }

    pub fn reset(&mut self) {
        self.memory = [0u8;30_000];
        self.memory_index = 0;
        self.loops = HashMap::new();
        self.pc = 0;
    }

    fn execution_reset(&mut self) {
        self.pc = 0;
        self.loops = HashMap::new();
    }

    fn map_loops(&mut self, operands: &Vec<char>) -> Result<(), InterpreterError> {
        let mut stack: Vec<usize> = Vec::new();

        for (i, &ch) in operands.iter().enumerate() {
            match ch {
                '[' => stack.push(i),
                ']' => {
                    match stack.pop() {
                        Some(start) => { self.loops.insert(i, start); },
                        None => return Err(InterpreterError::NoMatchingOpenLoop(i))
                    }
                }
                _ => {}
            }
        }

        if stack.iter().count() > 0 {
            return Err(InterpreterError::NoMatchingCloseLoop(stack.pop().unwrap()))
        }

        Ok(())
    }

    fn run_code(&mut self, chars: &Vec<char>) {
        let instructions_count = chars.len();
        
        while self.pc < instructions_count {
            let ch = chars[self.pc];
            self.process_operand(&ch);
            self.pc += 1;
        }
    }

    fn process_operand(&mut self, ch: &char) {
        match ch {
            '>' => self.next_byte(),
            '<' => self.prev_byte(),
            '+' => self.increment_byte(),
            '-' => self.decrement_byte(),
            ']' => self.resolve_loop(),
            '.' => self.print(),
            ',' => self.read(),
            _ => {}
        }
    }

    fn next_byte(&mut self) {
        self.memory_index += 1;
        if self.memory_index == self.memory.len() {
            self.memory_index = 0;
        }
    }

    fn prev_byte(&mut self) {
        if self.memory_index == 0 {
            self.memory_index = self.memory.len() - 1;
            return;
        } 
        self.memory_index -= 1;
    }

    fn increment_byte(&mut self) {
        if self.memory[self.memory_index] == u8::MAX {
            self.memory[self.memory_index] = u8::MIN;
            return;
        }
        self.memory[self.memory_index] += 1;
    }

    fn decrement_byte(&mut self) {
        if self.memory[self.memory_index] == u8::MIN {
            self.memory[self.memory_index] = u8::MAX;
            return;
        }
        self.memory[self.memory_index] -= 1;
    }

    fn resolve_loop(&mut self) {
        if self.memory[self.memory_index] > 0 {
            self.pc = self.loops.get(&self.pc).unwrap().to_owned()
        }
    }

    fn print(&self) {
        print!("{}", self.memory[self.memory_index] as char);
        io::stdout().flush().unwrap();
    }

    fn read(&mut self) {
        let mut buffer = [0u8;1];
        let result = io::stdin().read_exact(&mut buffer);

        self.memory[self.memory_index] = match result {
            Ok(_) => buffer[0],
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::u8;

    use super::*;

    #[test]
    fn should_return_error_when_not_closing_loop() {
        let mut i = BrainfuckInterpreter::new();
        let r = i.execute(&&String::from("["));
        assert!(matches!(
            r,
            Err(InterpreterError::NoMatchingCloseLoop(0))
        ));
    }

    #[test]
    fn should_return_error_when_not_opening_loop() {
        let mut i = BrainfuckInterpreter::new();
        let r = i.execute(&String::from("]"));
        assert!(matches!(
            r,
            Err(InterpreterError::NoMatchingOpenLoop(0))
        ));
    }

    #[test]
    fn should_execute() {
        let mut i = BrainfuckInterpreter::new();
        let r = i.execute(&String::from(">++[<++>-]"));
        assert!(matches!(
            r,
            Ok(())
        ));
    }

    #[test]
    fn should_increment_byte_index() {
        let mut i = BrainfuckInterpreter::new();
        i.execute(&String::from(">")).unwrap();
        assert_eq!(i.memory_index, 1);
    }

    #[test]
    fn should_wrap_while_incrementing_byte_index() {
        let mut i = BrainfuckInterpreter::new();
        i.memory_index = i.memory.len() - 1;
        i.execute(&String::from(">")).unwrap();
        assert_eq!(i.memory_index, 0);
    }

    #[test]
    fn should_decrement_byte_index() {
        let mut i = BrainfuckInterpreter::new();
        i.memory_index = 3;
        i.execute(&String::from("<")).unwrap();
        assert_eq!(i.memory_index, 2);
    }

    #[test]
    fn should_wrap_while_decrementing_byte_index() {
        let mut i = BrainfuckInterpreter::new();
        i.execute(&String::from("<")).unwrap();
        assert_eq!(i.memory_index, i.memory.len() - 1);
    }

    #[test]
    fn should_increment_byte() {
        let mut i = BrainfuckInterpreter::new();
        i.execute(&String::from("+")).unwrap();
        assert_eq!(i.memory[0], 1);
    }

    #[test]
    fn should_decrement_byte() {
        let mut i = BrainfuckInterpreter::new();
        i.memory[0] = 1;
        i.execute(&String::from("-")).unwrap();
        assert_eq!(i.memory[0], 0);
    }

    #[test]
    fn should_run_loop() {
        let mut i = BrainfuckInterpreter::new();
        i.execute(&String::from(">++[<+>-]")).unwrap();
        assert_eq!(i.memory[0], 2);
    }

    #[test]
    fn should_wrap_while_incrementing_byte_value() {
        let mut i = BrainfuckInterpreter::new();
        i.execute(&String::from("-")).unwrap();
        assert_eq!(i.memory[0], u8::MAX);
    }

    #[test]
    fn should_wrap_while_decrementing_byte_value() {
        let mut i = BrainfuckInterpreter::new();
        i.memory[0] = u8::MAX;
        i.execute(&String::from("+")).unwrap();
        assert_eq!(i.memory[0], u8::MIN);
    }
}