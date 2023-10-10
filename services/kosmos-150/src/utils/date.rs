extern crate chrono;

use chrono::prelude::*;

pub struct DateTime {
    ussr_months: Vec<String>
}

impl DateTime {
    pub fn new() -> Self {
        let ussr_months = vec![
            "Ленина".to_string(),
            "Маркса".to_string(),
            "революции".to_string(),
            "Свердлова".to_string(),
            "мая".to_string(),
            "Советской конституции".to_string(),
            "жатвы".to_string(),
            "мира".to_string(),
            "Коминтерна".to_string(),
            "Энгельса".to_string(),
            "великой революции".to_string(),
            "Сталина".to_string()
        ];
        
        DateTime {
            ussr_months: ussr_months
        }
    }

    pub fn get_current_date(&self) -> String {
        let current_date: NaiveDate = Utc::now().date_naive();
        format!("{} {}, 2093 год", 
            current_date.day0(), 
            self.ussr_months[current_date.month0() as usize]
        )
    }
}