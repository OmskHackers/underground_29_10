use chrono::Utc;

use crate::models::models::Order;
use crate::network::session::Session;
use crate::utils::rand::get_rand_element;
use crate::{models::models::{User, Spaceship, Flight, Spaceport}, error::AppError};

#[derive(Clone, Debug)]
pub struct AuthService;

impl AuthService {
    pub fn login(session: &mut Session, username: String, password: String) -> Result<String, AppError> {
        let res = User::find(
            username, 
            password,
        );
        match res {
            Ok(user) => {
                session.set_user_id(user.id);
                Ok("Вы вошли в систему. Путь к звёздам открыт, товарищ!".to_string())
            },
            Err(_e) => Err(AppError::new("пользователь не найден".to_string()))
        }
    }
    pub fn register(username: String, password: String) -> Result<String, AppError> {
        let res = User::create(username, password);
        match res {
            Ok(_user) => Ok("Вы зарегистрированы в службе Космос-150.".to_string()),
            Err(e) => Err(e)
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlightService;

impl FlightService {
    pub fn get_arriving_flights(_session: &Session) -> Result<String, AppError> {
        let flights_res = Flight::find_arriving();
        match flights_res {
            Ok(flights) => {
                let mut output = String::new();
                for flight in flights {
                    output.push_str(format!("{} {} {} {}\n", flight.spaceship_id, flight.from_spaceport_id, flight.to_spaceport_id, flight.departure).as_str());
                }
                Ok(output)
            }
            Err(e) => Err(e)
        }
    }
    pub fn generate_flights() -> Option<AppError> {
        let spaceships = Spaceship::find_all().expect("something went wrong while getting spaceships");
        let spaceports = Spaceport::find_all().expect("something went wrong while getting spaceports");
        
        for _i in 0..10 {
            let spaceship = get_rand_element(&spaceships);
            let departure_spaceport = get_rand_element(&spaceports);
            let arrival_spaceport = get_rand_element(&spaceports);

            let res = Flight::create(
                spaceship, 
                departure_spaceport, 
                arrival_spaceport, 
                Utc::now().naive_utc(),
            );
            match res {
                Ok(_res) => continue,
                Err(e) => return Some(e)
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct OrderService;

impl OrderService {
    pub fn create_order(session: &Session, flight_id: i32, occupied_seat: i32) -> Result<String, AppError> {
        if session.user_id.is_none() {
            return Err(AppError::new("пользователь не в системе".to_string()));
        }
        let user_id = session.user_id.unwrap();

        let flight_res = Flight::find_by_id(flight_id);
        match flight_res {
            Ok(flight) => {
                let res = Order::create(Order {
                    id: 0,
                    flight_id: flight_id,
                    user_id,
                    occupied_seat
                });
                match res {
                    Ok(_order) => {
                        Ok(format!("Создан заказ для пользователя {}, рейс {}-{}, дата и время вылета {}. Просим Вас явиться на космодром {} за час до вылета для получения билета. Спасибо!", 
                            user_id,
                            flight.from_spaceport_id,
                            flight.to_spaceport_id,
                            flight.departure,
                            flight.from_spaceport_id
                        ))
                    },
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
    pub fn get_user_orders(session: &Session) -> Result<String, AppError> {
        if session.user_id.is_none() {
            return Err(AppError::new("пользователь не в системе".to_string()));
        }
        let user_id = session.user_id.unwrap();

        let res = Order::find_orders_by_user(user_id);
        match res {
            Ok(orders) => {
                let mut output = String::new();
                output.push_str("Список заказов:\n");
                for idx in 0..orders.len() {
                    output.push_str(
                        format!(">> {}: {}-{}, {}, место {}", idx+1, orders[idx].flight_id, orders[idx].flight_id, orders[idx].id, orders[idx].occupied_seat)
                        .as_str()
                    );
                }
                Ok(output)
            },
            Err(e) => Err(e)
        }
    }
}