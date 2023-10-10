use std::fmt::format;

use crate::{services::auth_service::AuthService, models::error::ClientError};
use crate::services::flight_service::FlightService;
use crate::menu::transaction::Transaction;

#[derive(Clone)]
pub enum MenuTransaction {
    Input(Transaction),
    Output(String),
    Exit,
}

pub struct MenuItem {
    pub title: String,
    pub output: MenuTransaction,
}

pub struct Menu {
    auth_service: AuthService,
    flight_service: FlightService,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Self {
        let mut items = Vec::new();
        let auth_service = AuthService::new();
        let flight_service = FlightService::new();
        items.push(MenuItem { 
            title: ("Список прибывающих рейсов".to_owned()), 
            output: {MenuTransaction::Output("Хз мне пох".to_string())} }
        );
        items.push(MenuItem { 
            title: ("Список вылетающих рейсов".to_owned()), 
            output: (MenuTransaction::Output("Хз мне пох".to_string())) }
        );
        items.push(MenuItem { 
            title: ("Войти в систему".to_owned()), 
            output: (
                MenuTransaction::Input(Transaction::new(
                    vec!["входное имя".to_string(), "секретный ключ".to_string()]
                )))
            }
        );
        items.push(MenuItem { 
            title: ("Зарегистрироваться в системе".to_owned()), 
            output: (
                MenuTransaction::Input(Transaction::new(
                    vec!["входное имя".to_string(), "секретный ключ".to_string()]
                ))
            ) 
            }
        );
        items.push(MenuItem { 
            title: ("Историческая справка".to_owned()), 
            output: (MenuTransaction::Output(r#"
    В 2067 году, в честь столетия Октябрьской Революции, 
профессор Московского Государственного Университета (МГУ) по космической науке, Харитонов Иван Петрович, 
вдохновленный идеями социализма и научными достижениями, объединил свою страсть к космосу с горячим патриотизмом. 
    Совместно с группой своих преданных студентов, он решил создать службу заказа билетов в космическое пространство "Космос-150" в честь полуторавековой годовщины Революции.
Этот проект был непростым заданием, но идея была столь могущественной, 
что Иван Петрович и его студенты работали над ним без устали. 
Они верили, что каждый советский гражданин должен иметь возможность отправиться в космос и ощутить бескрайние просторы Вселенной,
точно так же, как это сделали первые космонавты страны.
С первых дней проекта, Иван Петрович и его студенты пропагандировали идеалы солидарности, равенства и братства, 
которые легли в основу Октябрьской Революции. Они убеждали, что "Космос-150" — это не просто путешествие, 
но возможность для каждого советского гражданина стать частью великой истории.
    Группа исследователей и инженеров разработала уникальные технологии и системы безопасности, 
которые позволили отправить советских граждан в космос с максимальным комфортом и безопасностью. 
Билеты на "Космос-150" стали доступными для широкой массы граждан, и каждый мог стать частью этой космической эпопеи.
Когда "Космос-150" был наконец запущен, весь мир восхищался советскими достижениями в космической индустрии. 
Этот проект был ярким символом советской мечты о звёздном будущем и великой истории борьбы за справедливость.
    И так, в 2067 году, в честь юбилея Октябрьской Революции, "Космос-150" открыл свои двери для советских граждан. 
Этот проект был воплощением идеалов социализма, а каждый билет на корабль "Космос-150" 
был билетом в великое будущее Советского Союза и космической эпохи. 
"Вместе мы покоряем космос!" - так громилось приглашение Ивана Петровича и его студентов, и они продолжали стремиться к звёздам вместе, как единое советское сообщество.

            "#.to_string())) }
        );
        items.push(MenuItem { 
            title: ("Выйти".to_owned()), 
            output: (MenuTransaction::Exit) }
        );

        Menu {
            auth_service,
            flight_service,
            items
        }
    }
    pub fn display_menu(&self) -> String {
        let mut menu = String::new();
        for i in 0..self.items.len() {
            menu.push_str(format!("> {idx}: {title}\n", idx=i+1, title=self.items[i].title).as_str());
        }
        menu
    }
    pub fn display_selected(&self, selected_item: usize) -> Result<MenuTransaction, String> {
        if selected_item <= self.items.len() {
            Ok(self.items[selected_item - 1].output.clone())
        } else {
            Err("Invalid selected item".to_string())
        }
    }
    pub fn commit(&self, selected: usize, user_inputs: Vec<String>) -> String {
        let res: Result<String, ClientError>;
        
        match selected {
            3 => {
                let username = user_inputs[0].clone();
                let password = user_inputs[1].clone();
                res = self.auth_service.login(username, password)
            }
            4 => {
                let username = user_inputs[0].clone();
                let password = user_inputs[1].clone();
                res = self.auth_service.register(username, password)
            }
            _ => {
                res = Ok("".to_string())
            }
        }

        match res {
            Ok(output)=> output,
            Err(err) => format!("ОШИБКА СО СТОРОНЫ ПОЛЬЗОВАТЕЛЯ: {:?}", err)
        }
    }
}