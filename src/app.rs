use std::cmp;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, style};

use super::{AppResult, Game, Grid};

/// A Rect is a tuple containing the (x-origin, y-origin, width, height) of a
/// rectangle.
type Rect = (i32, i32, i32, i32);

pub enum Sym {
    BoxTopLeft,
    BoxTopRight,
    BoxBottomLeft,
    BoxBottomRight,
    BoxVertical,
    BoxHorizontal,
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Sym::*;
        write!(
            f,
            "{}",
            match self {
                BoxTopLeft => '╔',
                BoxTopRight => '╗',
                BoxBottomLeft => '╚',
                BoxBottomRight => '╝',
                BoxVertical => '║',
                BoxHorizontal => '═',
            }
        )
    }
}

pub struct Menu {
    x0: u16,
    y0: u16,
    width: u16,
    height: u16,
    padding: u16,
    margin: u16,
}

impl Menu {
    pub fn new(x0: u16, y0: u16, width: u16, height: u16, padding: u16, margin: u16) -> Menu {
        Menu {
            x0,
            y0,
            width,
            height,
            padding,
            margin,
        }
    }

    pub fn draw_lines(&self) -> AppResult<Vec<String>> {
        let (x1, y1) = (self.x0 + self.width - 1, self.y0 + self.height - 1);
        let inner_width = cmp::min(0, self.width - 3) as usize;
        let mut lines = Vec::new();
        lines.push(format!(
            "{}{}{}",
            Sym::BoxTopLeft,
            Sym::BoxHorizontal.to_string().repeat(inner_width),
            Sym::BoxTopRight,
        ));
        for _ in self.y0 + 1..y1 {
            lines.push(format!(
                "{}{}{}",
                Sym::BoxVertical,
                " ".repeat(inner_width),
                Sym::BoxVertical
            ));
        }
        lines.push(format!(
            "{}{}{}",
            Sym::BoxBottomLeft,
            Sym::BoxHorizontal.to_string().repeat(inner_width),
            Sym::BoxBottomRight,
        ));
        Ok(lines)
    }
}

pub struct Config {
    pub stream_delay: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            stream_delay: Duration::from_millis(500),
        }
    }
}

pub struct App {
    game: Game,
    menu: Menu,
    opts: Config,
}

impl App {
    pub fn new(game: Game) -> App {
        App {
            game: game,
            menu: Menu::new(0, 0, 20, 20, 1, 1),
            opts: Default::default(),
        }
    }

    pub fn with_config(&mut self, opts: Config) -> &App {
        self.opts = opts;
        self
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> AppResult<App> {
        let mut f = File::open(path)?;
        let mut pattern = String::new();
        f.read_to_string(&mut pattern)?;
        let grid: Grid = pattern.parse()?;
        let game = Game::new(grid);
        Ok(App::new(game))
    }

    pub fn run(&mut self) -> AppResult<()> {
        let mut stdout = io::stdout().into_raw_mode()?;

        'Outer: while !self.game.is_over() {
            write!(stdout, "{}{}", clear::All, cursor::Hide)?;

            for (y, line) in self.game.draw().lines().enumerate() {
                write!(stdout, "{}{}", cursor::Goto(1, 1 + y as u16), line)?;
            }
            stdout.flush()?;

            for c in io::stdin().keys() {
                match c? {
                    Key::Char('q') | Key::Esc | Key::Ctrl('c') => break 'Outer,
                    Key::Char(' ') => break,
                    _ => (),
                }
            }

            self.game.tick();
        }
        self.teardown(&mut stdout)
    }

    pub fn run_as_stream(&mut self) -> AppResult<()> {
        let mut stdout = io::stdout();
        while !self.game.is_over() {
            for line in self.game.draw().lines() {
                write!(stdout, "{}\n", line)?;
            }
            write!(stdout, "\n")?;
            stdout.flush()?;
            self.game.tick();
            thread::sleep(self.opts.stream_delay);
        }
        Ok(())
    }

    pub fn teardown(&self, out: &mut RawTerminal<io::Stdout>) -> AppResult<()> {
        write!(out, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1),)?;
        Ok(())
    }
}
