use std::io::prelude::*;
use std::thread;

use super::{AppResult, Config, Game, Grid};

pub struct App {
    game: Game,
    opts: Config,
}

impl App {
    pub fn load() -> AppResult<App> {
        let opts = Config::load()?;
        let mut grid: Grid = opts.pattern.parse()?;
        grid.char_alive = opts.char_alive;
        grid.char_dead = opts.char_dead;
        let game = Game::new(grid);

        Ok(App { game, opts })
    }

    pub fn run<W: Write>(&mut self, out: &mut W) -> AppResult<()> {
        while !self.game.is_over() {
            for line in self.game.grid.to_string().lines() {
                write!(out, "{}\n", line)?;
            }
            write!(out, "\n")?;
            out.flush()?;
            self.game.tick();
            thread::sleep(self.opts.stream_delay);
        }
        Ok(())
    }
}
