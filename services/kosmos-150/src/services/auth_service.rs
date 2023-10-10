use crate::models::error::ClientError;

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        AuthService {  }
    }
    pub fn login(&self, username: String, password: String) -> Result<String, ClientError> {
        Ok("".to_string())
    }
    pub fn register(&self, username: String, password: String) -> Result<String, ClientError> {
        Ok("".to_string())
    }
}