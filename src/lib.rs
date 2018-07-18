#[macro_use]
#[cfg(test)]
extern crate maplit;
extern crate clap;
extern crate num_integer;
extern crate termion;

pub mod config;
pub mod game;
pub mod grid;
pub mod ui;

use std::error::Error;
use std::fmt;
use std::io;
use std::num;

pub use game::Game;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    ParseError,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::IOError => "IO error",
            ErrorKind::ParseError => "invalid input",
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    kind: ErrorKind,
    description: String,
    cause: Option<Box<Error>>,
}

impl AppError {
    pub fn new<E>(kind: ErrorKind, err: E) -> AppError
    where
        E: fmt::Display,
    {
        AppError {
            kind: kind,
            description: err.to_string(),
            cause: None,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conway: {}: {}", self.kind.as_str(), self.description)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|c| &**c)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: ErrorKind::IOError,
            description: error.to_string(),
            cause: Some(Box::new(error)),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: ErrorKind::ParseError,
            description: error.to_string(),
            cause: Some(Box::new(error)),
        }
    }
}
