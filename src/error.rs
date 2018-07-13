use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct AppError(pub String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conway: error: {}", self.0)
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError(error.to_string())
    }
}
