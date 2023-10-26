use chrono::{Utc, Duration};

use crate::models::models::Order;
use crate::network::session::Session;
use crate::utils::rand::get_rand_element;
use crate::{models::models::{User, Spaceship, Flight, Spaceport}, error::AppError};

#[derive(Clone, Debug)]
pub struct AuthService;

impl AuthService {
    pub fn login(session: &mut Session, username: String, password: String) -> Result<String, AppError> {
        let res = User::find(username);
        match res {
            Ok(user) => {
                if !password.eq(&user.password) {
                    return Err(AppError::new("пользователь не найден".to_string()))
                }
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
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    return Err(AppError::new("пользователь уже существует в системе".to_string()));
                }
                Err(e)
            }
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
                    let spaceship = Spaceship::find_by_id(flight.spaceship_id).expect("cannot find spaceship");
                    let departure_spaceport = Spaceport::find_by_id(flight.from_spaceport_id).expect("cannot find spaceport");
                    let arrival_spaceport = Spaceport::find_by_id(flight.to_spaceport_id).expect("cannot find spaceport");
                    
                    output.push_str(format!(
                        ">> | РЕЙC-{} | космолёт {}, {}({}, {}) ---> {}({}, {}), вылет в {}\n", 
                        flight.id,
                        spaceship.name, 
                        departure_spaceport.name,
                        departure_spaceport.location,
                        departure_spaceport.star_system, 
                        arrival_spaceport.name,
                        arrival_spaceport.location,
                        arrival_spaceport.star_system, 
                        flight.departure.format("%H:%M")
                    ).as_str());
                }
                Ok(output)
            }
            Err(e) => Err(e)
        }
    }
    pub fn generate_flights() -> Option<AppError> {
        let mut future = Utc::now().naive_local() + Duration::minutes(15);

        let spaceships = Spaceship::find_all().expect("something went wrong while getting spaceships");
        let spaceports = Spaceport::find_all().expect("something went wrong while getting spaceports");
        
        for _i in 0..15 {
            let spaceship = get_rand_element(&spaceships);
            let departure_spaceport = get_rand_element(&spaceports);
            let mut arrival_spaceport = get_rand_element(&spaceports);
            while departure_spaceport.id == arrival_spaceport.id {
                arrival_spaceport = get_rand_element(&spaceports);
            }

            let res = Flight::create(
                spaceship, 
                departure_spaceport, 
                arrival_spaceport, 
                future
            );
            future = future + Duration::minutes(1);
            match res {
                Ok(_res) => continue,
                Err(e) => return Some(e)
            }
        }
        None
    }
    pub fn remove_expired_flights() -> Option<AppError> {
        Flight::remove_expired_flights()
    }
}

#[derive(Clone, Debug)]
pub struct OrderService;

impl OrderService {
    pub fn create_order(session: &Session, flight_id: i32, occupied_seat: Option<i32>, comment: Option<String>) -> Result<String, AppError> {
        if session.user_id.is_none() {
            return Err(AppError::new("пользователь не в системе".to_string()));
        }
        let user_id = session.user_id.unwrap();

        let flight_res = Flight::find_by_id(flight_id);
        match flight_res {
            Ok(flight) => {
                let seat;
                if occupied_seat.is_some() {
                    seat = occupied_seat.unwrap();
                    let order_result = Order::find_flight_order_by_seat(flight_id, seat);
                    match order_result {
                        Ok(_) => {
                            return Err(AppError::new("место занято".to_string()));
                        },
                        Err(err) => {
                            if !err.to_string().contains("not found") {
                                return Err(err);
                            }
                        }
                    }
                } else {
                    let spaceship = Spaceship::find_by_id(flight.spaceship_id).expect("cannot find spaceship");
                    let seats_count = Order::count_flight_orders(flight_id).expect("failed to count seats");
                    if seats_count >= spaceship.seats_number.into() {
                        return Err(AppError::new("все места на данный рейс заняты".to_string()));
                    }
                    seat = seats_count as i32 + 1;
                }
                let res = Order::create(Order {
                    id: 0,
                    flight_id,
                    user_id,
                    occupied_seat: seat,
                    comment
                });
                match res {
                    Ok(_order) => {
                        let departure_spaceport = Spaceport::find_by_id(flight.from_spaceport_id).expect("cannot find spaceport");
                        let arrival_spaceport = Spaceport::find_by_id(flight.to_spaceport_id).expect("cannot find spaceport");
                        
                        Ok(format!("Заказ создан! РЕЙС-{} {} --> {}, вылет в {}. Просим Вас явиться на космодром '{}' за час до вылета для получения билета.", 
                            flight.id,
                            departure_spaceport.name,
                            arrival_spaceport.name,
                            flight.departure.format("%H:%M"),
                            departure_spaceport.name
                        ))
                    },
                    Err(e) => {
                        if e.to_string().contains("duplicate") {
                            return Err(AppError::new("заказ уже создан".to_string()));
                        }
                        Err(e)
                    }
                }
            }
            Err(e) => {
                if e.to_string().contains("not found") {
                    return Err(AppError::new("рейс не найден".to_string()))
                }
                Err(e)
            }
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
                if orders.len() == 0 {
                    return Ok("Ваш список заказов пока пустой...".to_string())
                }
                for order in orders {
                    let flight = Flight::find_by_id(order.flight_id).expect("cannot find flight");
                    let departure_spaceport = Spaceport::find_by_id(flight.from_spaceport_id).expect("cannot find spaceport");
                    let arrival_spaceport = Spaceport::find_by_id(flight.to_spaceport_id).expect("cannot find spaceport");

                    output.push_str(
                        format!(
                            ">> ЗАКАЗ-{}: | РЕЙC-{} | {}({}, {}) ---> {}({}, {}), вылет в {}, место {}, \n", 
                            order.id,
                            order.flight_id,
                            departure_spaceport.name,
                            departure_spaceport.location,
                            departure_spaceport.star_system, 
                            arrival_spaceport.name,
                            arrival_spaceport.location,
                            arrival_spaceport.star_system, 
                            flight.departure.format("%H:%M"),
                            order.occupied_seat
                        ).as_str()
                    );
                }
                Ok(output)
            },
            Err(e) => Err(e)
        }
    }
    pub fn get_order_seats(session: &Session, order_id: i32) -> Result<String, AppError> {
        if session.user_id.is_none() {
            return Err(AppError::new("пользователь не в системе".to_string()));
        }
        let user_id = session.user_id.unwrap();

        let order_res = Order::find_by_id(user_id, order_id);
        match order_res {
            Ok(order) => {
                let spaceship = Spaceship::find_by_order(order_id).expect("cannot find spaceship");
                let flight = Flight::find_by_id(order.flight_id).expect("cannot find flight");
                let departure_spaceport = Spaceport::find_by_id(flight.from_spaceport_id).expect("cannot find spaceport");
                let arrival_spaceport = Spaceport::find_by_id(flight.to_spaceport_id).expect("cannot find spaceport");

                let mut output = String::new();
                output.push_str(format!(
                    ">> ЗАКАЗ-{}: космолёт {}, место {}, {} ---> {}, вылет в {}",
                    order.id,
                    spaceship.name,
                    order.occupied_seat,
                    departure_spaceport.name,
                    arrival_spaceport.name,
                    flight.departure.format("%H:%M")
                ).as_str());
                if order.comment.is_some() {
                    let comment = order.comment.unwrap();
                    output.push_str(
                        format!(", дополнительные пожелания: {}\n", 
                        comment
                    ).as_str());
                } else {
                    output.push_str("\n");
                }
                let occupied_seats_res = Order::get_order_occupied_seats(order_id);
                match occupied_seats_res {
                    Ok(occupied_seats) => {
                        output.push_str(format!(
                            "Занятые места:",
                        ).as_str());
                        for occupied_seat in occupied_seats {
                            output.push_str(format!(
                                " {}",
                                occupied_seat
                            ).as_str());
                        }
                        Ok(output)
                    },
                    Err(e) => Err(e)
                }
            },
            Err(_) => Err(AppError::new("заказ не найден".to_string()))
        }
    }
}
