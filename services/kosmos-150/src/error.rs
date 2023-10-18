use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ClientError {
    message: String
}

impl ClientError {
    pub fn new(message: String) -> Self {
        ClientError {
            message
        }
    }
}

impl error::Error for ClientError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "Client Error")
    }
}

#[derive(Debug, Clone)]
pub struct ServerError {
    message: String
}

impl ServerError {
    pub fn new(message: String) -> Self {
        ServerError {
            message
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "Server Error")
    }
}