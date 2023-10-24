extern crate chrono;

use chrono::prelude::*;

const USSR_MONTHS: [&'static str; 12] = [
    "Ленина",
    "Маркса",
    "революции",
    "Свердлова",
    "мая",
    "Советской конституции",
    "жатвы",
    "мира",
    "Коминтерна",
    "Энгельса",
    "великой революции",
    "Сталина"
];

pub struct UssrDateTime;

impl UssrDateTime {
    pub fn get_current_date() -> String {
        let current_date: NaiveDate = Utc::now().date_naive();
        format!("{} {}, {}", 
            current_date.day0(), 
            USSR_MONTHS[current_date.month0() as usize],
            Local::now().time().format("%H:%M")
        )
    }
}