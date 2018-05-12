use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ChessError {
    details: String,
}

impl ChessError {
    pub fn new(msg: &str) -> ChessError {
        ChessError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ChessError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for ChessError {
    fn from(err: ParseIntError) -> Self {
        ChessError::new(err.description())
    }
}
