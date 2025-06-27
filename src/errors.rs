use std::error::Error;
use std::{fmt, process};

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    InvalidInput(String),
    InvalidCommand(String),
    InvalidFlag(String),
    Unknown,
}

impl AppError {
    pub fn exit(&self) {
        eprintln!("{}", self);
        process::exit(1);
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "Error: IO error: {}", e),
            AppError::InvalidInput(msg) => write!(f, "Error: Invalid input: {}", msg),
            AppError::InvalidCommand(cmd) => write!(f, "Error: Invalid command: {}", cmd),
            AppError::InvalidFlag(flag) => write!(f, "Error: Invalid flag: {}", flag),
            AppError::Unknown => write!(f, "Error: Unknown error"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::IoError(e) => Some(e),
            _ => None,
        }
    }
}
