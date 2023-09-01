use std::error::Error;
use std::fmt;

//------------Topic Error--------------------
#[derive(Debug, PartialEq)]
pub struct TopicError {
    details: String,
}

impl TopicError {
    pub fn new(msg: &str) -> TopicError {
        TopicError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TopicError {
    fn description(&self) -> &str {
        &self.details
    }
}


//------------Protocol Error--------------------
#[derive(Debug, PartialEq)]
pub struct ProtocolError {
    details: String,
}

impl ProtocolError {
    pub fn new(msg: &str) -> ProtocolError {
        ProtocolError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ProtocolError {
    fn description(&self) -> &str {
        &self.details
    }
}

//------------Server Error--------------------
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

//------------Consumer Error--------------------
#[derive(Debug, PartialEq)]
pub struct ConsumerError {
    details: String,
}

impl ConsumerError {
    pub fn new(msg: &str) -> ConsumerError {
        ConsumerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ConsumerError {
    fn description(&self) -> &str {
        &self.details
    }
}

//------------Producer Error--------------------
#[derive(Debug, PartialEq)]
pub struct ProducerError {
    details: String,
}

impl ProducerError {
    pub fn new(msg: &str) -> ProducerError {
        ProducerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ProducerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ProducerError {
    fn description(&self) -> &str {
        &self.details
    }
}