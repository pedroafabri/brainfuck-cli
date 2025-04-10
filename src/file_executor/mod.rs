use std::fs;
use crate::interpreter;

pub fn run_file(path: &String) {
    match read_file_content(path) {
        Ok(code) => { run_code(&code); }
        Err(e) => { eprintln!("Failed to read file \"{path}\": {e}"); }
    }
}

fn read_file_content(path: &String) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn run_code(code: &String) {
    let mut interpreter = interpreter::BrainfuckInterpreter::new();
    match interpreter.execute(code) {
        Err(e) => { eprintln!("{e}"); }
        _ => {}
    }
}