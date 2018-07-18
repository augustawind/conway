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

pub use game::Game;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ParseInt(std::num::ParseIntError),
    ParseChar(std::char::ParseCharError),
    IO(io::Error),
    Msg(String),
    WithCause(Box<AppError>, Box<Error + 'static>),
}

impl AppError {
    pub fn with_cause<E>(self, err: E) -> AppError
    where
        E: Error + 'static,
    {
        AppError::WithCause(Box::new(self), Box::new(err))
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (prefix, msg) = match self {
            AppError::ParseInt(e) => ("expected an integer", e.to_string()),
            AppError::ParseChar(e) => ("expected a single character", e.to_string()),
            AppError::IO(e) => ("IO failed", e.to_string()),
            AppError::Msg(e) => ("invalid input", e.to_string()),
            AppError::WithCause(e, _) => return e.fmt(f),
        };
        write!(f, "conway: {}: {}", prefix, msg)
    }
}

impl Error for AppError {
    fn cause(&self) -> Option<&Error> {
        if let AppError::WithCause(_, ref err) = *self {
            Some(&**err)
        } else {
            None
        }
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Msg(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IO(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseInt(error)
    }
}

impl From<std::char::ParseCharError> for AppError {
    fn from(error: std::char::ParseCharError) -> Self {
        AppError::ParseChar(error)
    }
}
