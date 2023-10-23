use crate::{error::AppError, network::session::Session};

pub struct Request {
    user_data: Vec<String>,
}

impl Request {
    pub fn new(user_data: Vec<String>) -> Self {
        Request { 
            user_data,
        }
    }
    pub fn get_user_data(&self) -> &Vec<String> {
        &self.user_data
    }
}

#[derive(Clone)]
pub struct Transaction {
    data_to_enter: Vec<String>,
    user_inputs: Vec<String>,
    commit_fn: fn(&mut Session, Request) -> Result<String, AppError>
}

impl Transaction {
    pub fn new(data_to_enter: Vec<String>, commit_fn: fn(&mut Session, Request) -> Result<String, AppError>) -> Self {
        Transaction { 
            data_to_enter, 
            commit_fn,
            user_inputs: Vec::new() 
        }
    }
    pub fn ask(&self) -> String {
        format!("Введите {}:\n", self.data_to_enter[self.user_inputs.len()])
    }
    pub fn enter(&mut self, data: String) {
        self.user_inputs.push(data);
    }
    pub fn done(&self) -> bool {
        self.user_inputs.len() == self.data_to_enter.len()
    }
    pub fn commit(&self, session: &mut Session) -> String {
        let res = (self.commit_fn)(session, Request { user_data: self.data_to_enter.clone() });
        match res {
            Ok(output) => output,
            Err(e) => format!("ОШИБКА НА СТОРОНЕ ПОЛЬЗОВАТЕЛЯ: {}\n", e.to_string())
        }
    }
    pub fn get_user_inputs(&self) -> Vec<String> {
        self.user_inputs.clone()
    }
}