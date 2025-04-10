mod interpreter;
mod file_executor;
mod repl;

use std::{env, process};
use repl::ReplSession;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => { ReplSession::new().start(); },
        2 => { process_argument(&args[1]); },
        _ => { eprintln!("Incorrect usage. Use -h or --help for instructions.") }
    }
    process::exit(0);
    
}

fn process_argument(arg: &String) {
    match arg.as_str() {
        "-h" | "--help" => print_help(),
        _ => file_executor::run_file(arg),
    }
}

fn print_help() {
    println!(
        "\
brainfuck v{} - A simple Brainfuck interpreter written in Rust

USAGE:
    brainfuck [file]

OPTIONS:
    -h, --help      Show this help message

BEHAVIOR:
    If no [file] is provided, the interpreter will launch in REPL mode,
    allowing you to type and run Brainfuck commands interactively.
    If a [file] is provided, the interpreter will execute the code
    from the given file.

EXAMPLES:
    brainfuck               # Start REPL mode
    brainfuck hello.bf      # Run 'hello.bf' file
    brainfuck --help        # Show this help message
", env!("CARGO_PKG_VERSION"));
}