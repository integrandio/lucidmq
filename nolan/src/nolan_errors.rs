use std::fmt;
use std::error::Error;

//------------Index Error--------------------
#[derive(Debug)]
pub struct IndexError {
    details: String
}

impl IndexError {
    pub fn new(msg: &str) -> IndexError {
        IndexError{details: msg.to_string()}
    }
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for IndexError {
    fn description(&self) -> &str {
        &self.details
    }
}

//------------Segment Error--------------------
#[derive(Debug, PartialEq)]
pub struct SegmentError {
    details: String
}

impl SegmentError {
    pub fn new(msg: &str) -> SegmentError {
        SegmentError{details: msg.to_string()}
    }
}

impl fmt::Display for SegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for SegmentError {
    fn description(&self) -> &str {
        &self.details
    }
}

//------------Commitlog Error--------------------
// #[derive(Debug, PartialEq)]
// pub struct CommitlogError {
//     details: String
// }

// impl CommitlogError {
//     pub fn new(msg: &str) -> CommitlogError {
//         CommitlogError{details: msg.to_string()}
//     }
// }

// impl fmt::Display for CommitlogError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{}",self.details)
//     }
// }

// impl Error for CommitlogError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }