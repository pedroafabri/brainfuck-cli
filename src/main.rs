mod interpreter;

fn main() {
    let code = "[,.]";
    let mut i = interpreter::BrainfuckInterpreter::new();
    i.execute(code.to_string()).unwrap();
}
