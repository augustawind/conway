use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};

use super::{AppResult, Game, Grid};

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn run_from_path<P: AsRef<Path>>(&self, path: P) -> AppResult<()> {
        let mut f = File::open(path)?;
        let mut pattern = String::new();
        f.read_to_string(&mut pattern)?;
        let grid: Grid = pattern.parse()?;
        let game = Game::new(grid);
        self.run(game)
    }

    pub fn run(&self, game: Game) -> AppResult<()> {
        let mut stdout = io::stdout().into_raw_mode()?;

        'Outer: for output in game {
            write!(stdout, "{}{}", clear::All, cursor::Hide)?;

            for (y, line) in output.lines().enumerate() {
                write!(stdout, "{}{}", cursor::Goto(1, y as u16 + 1), line)?;
            }
            stdout.flush()?;

            for c in io::stdin().keys() {
                match c? {
                    Key::Char('q') | Key::Esc | Key::Ctrl('c') => break 'Outer,
                    Key::Char(' ') => break,
                    _ => (),
                }
            }
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
