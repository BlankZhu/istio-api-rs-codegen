use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct KindError {
    detail: String,
}

impl KindError {
    pub fn new(msg: &str) -> KindError {
        KindError {
            detail: msg.to_string(),
        }
    }
}

impl fmt::Display for KindError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.detail)
    }
}

impl Error for KindError {
    fn description(&self) -> &str {
        &self.detail
    }
}