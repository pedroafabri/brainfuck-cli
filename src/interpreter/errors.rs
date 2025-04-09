use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum InterpreterError {
    #[error("Found ']' at position {0}, but no matching '[' was found.")]
    NoMatchingOpenLoop(usize),

    #[error("Found '[' at position {0}, but no matching ']' was found.")]
    NoMatchingCloseLoop(usize),
}