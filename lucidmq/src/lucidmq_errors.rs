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