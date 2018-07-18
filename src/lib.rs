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

#[derive(Debug, Clone)]
pub struct AppError(pub String);

impl AppError {
    pub fn new(s: &str) -> AppError {
        AppError(s.to_owned())
    }
}

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
