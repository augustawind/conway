use std::mem;
use std::thread;

use config::{ConfigSet, GameConfig};
use grid::{Cell, Grid};
use AppResult;

pub struct GameIter<'a>(&'a mut Game);

impl<'a> Iterator for GameIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_over() {
            return None;
        }
        self.0.tick();
        thread::sleep(self.0.opts.tick_delay);
        Some(self.0.draw())
    }
}

/// Game holds the high-level gameplay logic.
#[derive(Debug)]
pub struct Game {
    grid: Grid,
    swap: Grid,
    opts: GameConfig,
}

impl Game {
    pub fn load() -> AppResult<Game> {
        let config = ConfigSet::from_env()?;
        let grid = Grid::from_config(config.grid)?;
        Ok(Game::new(grid, config.game))
    }

    pub fn new(grid: Grid, opts: GameConfig) -> Game {
        let mut swap = grid.clone();
        swap.clear();

        Game { grid, swap, opts }
    }

    pub fn iter(&mut self) -> GameIter {
        GameIter(self)
    }

    pub fn draw(&self) -> String {
        self.grid.to_string()
    }

    /// Return whether the Game is over. This happens with the Grid is empty.
    pub fn is_over(&self) -> bool {
        self.grid.is_empty()
    }

    /// Progress the Game of Life forward.
    ///
    /// `tick` applies the rules of game to each individual Cell, killing some and reviving others.
    pub fn tick(&mut self) {
        for cell in self.grid.active_cells() {
            if self.survives(&cell) {
                self.swap.set_alive(cell);
            }
        }
        self.grid.clear();
        mem::swap(&mut self.grid, &mut self.swap);
    }

    /// Survives returns whether the given Cell survives an application of the Game Rules.
    pub fn survives(&self, cell: &Cell) -> bool {
        let live_neighbors = self.grid.live_neighbors(cell);
        if self.grid.is_alive(cell) {
            match live_neighbors {
                2 | 3 => true,
                _ => false,
            }
        } else {
            match live_neighbors {
                3 => true,
                _ => false,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_survives_blinker() {
        let game = Game::new(
            Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)], Default::default()),
            Default::default(),
        );
        assert!(
            game.survives(&Cell(1, 1)),
            "a live cell with 2 live neighbors should survive"
        );
        assert!(
            game.survives(&Cell(0, 1)),
            "a dead cell with 3 live neighbors should survive"
        );
        assert!(
            game.survives(&Cell(2, 1)),
            "a dead cell with 3 live neighbors should survive"
        );
        assert!(
            !game.survives(&Cell(1, 0)),
            "a live cell with < 2 live neighbors should die"
        );
        assert!(
            !game.survives(&Cell(1, 2)),
            "a live cell with < 2 live neighbors should die"
        );
    }
}
