use std::mem;

use super::grid::{Cell, Grid};

/// Game holds the high-level gameplay logic.
#[derive(Debug)]
pub struct Game {
    pub grid: Grid,
    swap: Grid,
}

impl Game {
    /// Create a new Game from a Grid.
    pub fn new(grid: Grid) -> Game {
        let mut swap = grid.clone();
        swap.clear();
        Game { grid, swap }
    }

    /// Return whether the Game is over. This happens with the Grid is empty.
    pub fn is_over(&self) -> bool {
        self.grid.is_empty()
    }

    /// Return the Game as a string for output.
    pub fn draw(&self) -> String {
        self.grid.to_string()
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

impl Iterator for Game {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.grid.is_empty() {
            self.tick();
            Some(self.draw())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_survives_blinker() {
        let game = Game::new(Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)], 0, 0));
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
