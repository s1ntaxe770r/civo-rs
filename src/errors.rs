use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HTTPError {
    pub code: u16,
    pub message: String,
}

impl HTTPError {
    pub fn new(code: u16, message: &str) -> HTTPError {
        HTTPError {
            code,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for HTTPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP error {} : {}", self.code, self.message)
    }
}

impl Error for HTTPError {}
