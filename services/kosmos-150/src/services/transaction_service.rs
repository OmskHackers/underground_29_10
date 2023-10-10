pub struct TransactionService {

}

impl TransactionService {
    pub fn new() -> Self {
        TransactionService {

        }
    }
    pub fn start_login_transaction(&self) -> Transaction {
        Transaction::new(vec!["имя пользователя".to_string(), "пароль".to_string()])
    }
}