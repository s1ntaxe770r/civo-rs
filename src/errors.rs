use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct HTTPError {
    pub Code: i32,
    pub Message: String,
}


impl HTTPError {
    pub fn new(code:i32,message: &str) -> HTTPError{
        HTTPError { Code: code, Message: message.to_string() }
    } 
}

impl fmt::Display for HTTPError{ 
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"HTTP error {} : {}",self.Code, self.Message)
    }
}

impl Error for HTTPError {}