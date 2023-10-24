use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AppError {
    message: String
}

impl AppError {
    pub fn new(message: String) -> Self {
        AppError {
            message
        }
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}