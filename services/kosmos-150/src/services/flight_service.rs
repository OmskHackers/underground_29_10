use crate::error::ClientError;

#[derive(Clone)]
pub struct FlightService {
}

impl FlightService {
    pub fn new() -> FlightService {
        FlightService { }
    }
    pub fn get_arriving_flights(&self) -> Result<String, ClientError> {
        Ok("".to_owned())
    }
    pub fn get_departure_flights(&self) -> Result<String, ClientError> {
        Ok("".to_owned())
    }
    pub fn order_flight(&self) -> Result<String, ClientError> {
        Ok("".to_owned())
    }
    pub fn cancel_order(&self) -> Result<String, ClientError> {
        Ok("".to_owned())
    }
}