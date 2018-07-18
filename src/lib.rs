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
pub enum AppError {
    IO(io::Error),
    ParseInt(num::ParseIntError),
    Data(Box<Error>),
}

impl AppError {
    pub fn from_err<E>(err: E) -> AppError
    where
        E: Error + 'static,
    {
        AppError::Data(Box::new(err))
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (prefix, msg) = match self {
            AppError::IO(e) => ("IO failed", e.to_string()),
            AppError::ParseInt(e) => ("number is not a valid integer", e.to_string()),
            AppError::Data(e) => ("invalid input", e.to_string()),
        };
        write!(f, "conway: {}: {}", prefix, msg)
    }
}

impl Error for AppError {}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Data(From::from(error))
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IO(error)
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError::ParseInt(error)
    }
}
