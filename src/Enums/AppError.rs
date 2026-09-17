use std::fmt;

#[derive(Debug)]
pub enum AppError{
    RuntimeError(String),
}

impl fmt::Display for AppError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RuntimeError(msg)=>write!(f, "{}", msg),
        }
    }
}