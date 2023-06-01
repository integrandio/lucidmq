use std::error::Error;
use std::fmt;

//------------Server Error--------------------
#[derive(Debug)]
pub struct ServerError {
    details: String,
}

impl ServerError {
    pub fn new(msg: &str) -> ServerError {
        ServerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        &self.details
    }
}

//------------Broker Error--------------------
#[derive(Debug)]
pub struct BrokerError {
    details: String,
}

impl BrokerError {
    pub fn new(msg: &str) -> BrokerError {
        BrokerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for BrokerError {
    fn description(&self) -> &str {
        &self.details
    }
}