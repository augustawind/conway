use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};

use super::{AppError, Game, Grid};

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn run_from_path<P: AsRef<Path>>(&self, path: P) -> Result<(), AppError> {
        let mut f = File::open(path)?;
        let mut pattern = String::new();
        f.read_to_string(&mut pattern)?;
        let grid: Grid = pattern.parse()?;
        let game = Game::new(grid);
        self.run(game)
    }

    pub fn run(&self, game: Game) -> Result<(), AppError> {
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode()?;

        for output in game {
            println!("{}", output);
            thread::sleep(Duration::from_millis(1000));
            break;
        }
        write!(
            stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1),
        )?;
        Ok(())
    }
}
