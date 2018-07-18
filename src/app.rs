use std::cmp;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::thread;

use num_integer::div_floor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};

use super::{AppResult, Config, Game, Grid};

/// A Rect is a tuple struct containing the (x-origin, y-origin, width, height) of a rectangle.
#[derive(Debug)]
pub struct Rect {
    x0: u16,
    y0: u16,
    width: u16,
    height: u16,
}

impl Rect {
    /// Create a new Rect.
    pub fn new(x0: u16, y0: u16, width: u16, height: u16) -> Rect {
        Rect {
            x0: x0,
            y0: y0,
            width: width,
            height: height,
        }
    }

    /// Retrieve the Rect's origin X, origin Y, width and height.
    pub fn shape(&self) -> (u16, u16, u16, u16) {
        (self.x0, self.y0, self.width, self.height)
    }

    /// Retrieve the Rect's origin X and Y and it's opposite X and Y.
    pub fn coords(&self) -> (u16, u16, u16, u16) {
        let (x0, y0, width, height) = self.shape();
        (x0, y0, (x0 + width - 1), (y0 + height - 1))
    }

    pub fn resized(&self, dx: i16, dy: i16) -> Rect {
        let (x0, y0, x1, y1) = self.coords();
        let (dx, dy) = (div_floor(dx, 2), div_floor(dy, 2));
        let (x0, y0, x1, y1) = (
            x0 as i16 - dx,
            y0 as i16 - dy,
            x1 as i16 + dx,
            y1 as i16 + dy,
        );

        if x0 >= x1 || y0 >= y1 {
            panic!("cannot shrink Rect more than its own size");
        } else if x0 <= 0 || y0 <= 0 {
            panic!("cannot expand Rect out of bounds");
        }

        Rect {
            x0: x0 as u16,
            y0: y0 as u16,
            width: (x1 - x0 + 1) as u16,
            height: (y1 - y0 + 1) as u16,
        }
    }
}

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

pub trait Widget {
    fn draw(&self) -> String;
    fn rect(&self) -> &Rect;

    fn margin(&self) -> u16 {
        1
    }

    fn padding(&self) -> u16 {
        1
    }

    fn draw_box(&self) -> String {
        let (_, y0, width, height) = self.rect().shape();
        let y1 = y0 + height - 1;
        let inner_width = cmp::max(0, width - 3) as usize;
        let mut s = String::new();
        s.push_str(&format!(
            "{}{}{}\n",
            Sym::BoxTopLeft,
            Sym::BoxHorizontal.to_string().repeat(inner_width),
            Sym::BoxTopRight,
        ));
        for _ in y0 + 1..y1 {
            s.push_str(&format!(
                "{}{}{}\n",
                Sym::BoxVertical,
                " ".repeat(inner_width),
                Sym::BoxVertical
            ));
        }
        s.push_str(&format!(
            "{}{}{}\n",
            Sym::BoxBottomLeft,
            Sym::BoxHorizontal.to_string().repeat(inner_width),
            Sym::BoxBottomRight,
        ));
        s
    }

    fn render_lines<'a, W, I>(&self, out: &mut W, lines: I, rect: &Rect) -> AppResult<()>
    where
        W: Write,
        I: Iterator<Item = &'a str>,
    {
        let (x0, y0, _, height) = rect.shape();

        for (y, line) in lines.take(height as usize).enumerate() {
            write!(out, "{}{}", cursor::Goto(x0 + 1, y0 + 1 + y as u16), line)?;
        }

        Ok(())
    }

    fn render<W: Write>(&self, out: &mut W) -> AppResult<()> {
        let rect = self.rect();
        self.render_lines(out, self.draw_box().lines(), &rect)?;
        let inner_rect = &rect.resized(-2, -2);
        self.render_lines(out, self.draw().lines(), &inner_rect)?;

        Ok(())
    }
}

static MENU_CMDS: &'static str = "
%     COMMANDS     %
--------------------
next    -->    Space
quit    -->    q/Esc
";

pub struct Menu {
    rect: Rect,
    padding: u16,
    margin: u16,
}

impl Menu {
    pub fn new(rect: Rect, padding: u16, margin: u16) -> Menu {
        Menu {
            rect,
            padding,
            margin,
        }
    }
}

impl Widget for Menu {
    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn margin(&self) -> u16 {
        self.margin
    }

    fn padding(&self) -> u16 {
        self.padding
    }

    fn draw(&self) -> String {
        MENU_CMDS.trim().to_string()
    }
}

impl Widget for Game {
    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn draw(&self) -> String {
        self.grid.to_string()
    }
}

pub struct App {
    game: Game,
    menu: Menu,
    opts: Config,
}

impl App {
    pub fn load() -> AppResult<App> {
        let config = Config::load()?;
        let menu = Menu::new(Rect::new(0, 0, 23, 20), 1, 1);
        let mut grid: Grid = config.pattern.parse()?;
        grid.char_alive = config.char_alive;
        grid.char_dead = config.char_dead;
        let mut game = Game::new(grid);
        game.rect = {
            let (x0, y0, width, height) = menu.rect().shape();
            Rect::new(x0 + width - 1, y0, 40, height)
        };

        Ok(App {
            game,
            menu,
            opts: config,
        })
    }

    pub fn render(&mut self, stdout: &mut io::StdoutLock) -> AppResult<()> {
        self.menu.render(stdout)?;
        self.game.render(stdout)?;
        Ok(())
    }

    pub fn run(&mut self) -> AppResult<()> {
        if self.opts.raw {
            self.run_as_stream()
        } else {
            self.run_as_app()
        }
    }

    pub fn run_as_app(&mut self) -> AppResult<()> {
        let stdout = io::stdout().into_raw_mode()?;
        let mut stdout = stdout.lock();

        'Outer: while !self.game.is_over() {
            write!(stdout, "{}{}", clear::All, cursor::Hide)?;

            self.render(&mut stdout)?;
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

    pub fn teardown<W: Write>(&self, mut out: W) -> AppResult<()> {
        write!(out, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1),)?;
        Ok(())
    }
}
