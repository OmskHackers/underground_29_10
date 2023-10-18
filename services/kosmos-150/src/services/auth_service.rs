use crate::{models::models::User, error::ClientError};

#[derive(Clone, Debug)]
pub struct AuthService;

impl AuthService {
    pub fn login(username: String, password: String) -> Result<String, ClientError> {
        let res = User::find(
            username, 
            password,
        );
        match res {
            Ok(_user) => Ok("Вы вошли в систему. Путь к звёздам открыт, товарищ!".to_string()),
            Err(e) => Err(e)
        }
    }
    pub fn register(username: String, password: String) -> Result<String, ClientError> {
        let res = User::create(User {
            id: 0,
            username: username, 
            password: password,
        });
        match res {
            Ok(_user) => Ok("Вы зарегистрированы в службе Космос-150.".to_string()),
            Err(e) => Err(e)
        }
    }
}