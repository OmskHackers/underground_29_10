pub struct Request {
    user_data: Vec<String>
}

impl Request {
    pub fn new(user_data: Vec<String>) -> Self {
        Request { 
            user_data: user_data 
        }
    }
    pub fn get_user_data(&self) -> &Vec<String> {
        &self.user_data
    }
}

#[derive(Clone)]
pub struct Transaction {
    data_to_enter: Vec<String>,
    user_inputs: Vec<String>
}

impl Transaction {
    pub fn new(data_to_enter: Vec<String>) -> Self {
        Transaction { 
            data_to_enter, 
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
    pub fn get_user_inputs(&self) -> Vec<String> {
        self.user_inputs.clone()
    }
}