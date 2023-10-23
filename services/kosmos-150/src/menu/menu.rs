use crate::{menu::transaction::Transaction, services::services::{AuthService, FlightService, OrderService}, network::session::Session, error::AppError};

#[derive(Clone)]
pub enum MenuTransaction {
    Input(Transaction),
    Output(fn(&Session) -> Result<String, AppError>),
    Exit,
}

#[derive(Clone)]
pub struct MenuItem {
    pub guest_access: bool,
    pub auth_access: bool,
    pub title: String,
    pub output: MenuTransaction,
}

#[derive(Clone)]
pub struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Self {
        let mut items = Vec::new();

        items.push(MenuItem { 
            guest_access: true,
            auth_access: true,
            title: ("Список прибывающих рейсов".to_owned()), 
            output: {MenuTransaction::Output(|session| FlightService::get_arriving_flights(session))} }
        );
        items.push(MenuItem { 
            guest_access: false,
            auth_access: true,
            title: ("Создать заказ".to_string()), 
            output: (MenuTransaction::Input(Transaction::new(
                vec!["идентификатор рейса".to_string(), "место (нажмите ВВОД, чтобы выбрать случайное)".to_string()], 
                |session, request| {
                    OrderService::create_order(session, flight, occupied_seat)
                }
            ))) }
        );
        items.push(MenuItem { 
            guest_access: false,
            auth_access: true,
            title: ("Ваши заказы".to_string()), 
            output: MenuTransaction::Output(|session| OrderService::get_user_orders(session)) }
        );
        items.push(MenuItem { 
            guest_access: true,
            auth_access: false,
            title: ("Войти в систему".to_owned()), 
            output: (
                MenuTransaction::Input(Transaction::new(
                    vec!["входное имя".to_string(), "секретный ключ".to_string()],
                    |session, req| {
                        let user_data = req.get_user_data();
                        let username = user_data[0].clone();
                        let password = user_data[1].clone();
                        AuthService::login(session, username, password)
                    }
                )))
            }
        );
        items.push(MenuItem { 
            guest_access: true,
            auth_access: false,
            title: ("Зарегистрироваться в системе".to_owned()), 
            output: (
                MenuTransaction::Input(Transaction::new(
                    vec!["входное имя".to_string(), "секретный ключ".to_string()],
                    |_session, req| {
                        let user_data = req.get_user_data();
                        let username = user_data[0].clone();
                        let password = user_data[1].clone();
                        AuthService::register(username, password)
                    }
                ))
            ) 
            }
        );
        items.push(MenuItem {
            guest_access: true,
            auth_access: true, 
            title: ("Историческая справка".to_owned()), 
            output: (MenuTransaction::Output(|_| Ok(r#"
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
            "#.to_string()))) }
        );
        items.push(MenuItem { 
            guest_access: true,
            auth_access: true,
            title: ("Выйти".to_string()), 
            output: (MenuTransaction::Exit) }
        );

        Menu {
            items
        }
    }
    pub fn display_menu(&self, session: &Session) -> String {
        let mut menu = String::new();
        let is_authorized = session.user_id.is_some();
        menu.push_str("\n");
        for i in 0..self.items.len() {
            if (is_authorized && self.items[i].auth_access )|| (!is_authorized && self.items[i].guest_access) {
                menu.push_str(format!("> {idx}: {title}\n", idx=i+1, title=self.items[i].title).as_str());
            }
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
}