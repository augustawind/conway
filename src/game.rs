use std::default::Default;
use std::mem;
use std::thread;
use std::time::Duration;

use super::grid::{Cell, Grid};

pub struct Game {
    grid: Grid,
    swap: Grid,
}

impl Game {
    pub fn new(grid: Grid) -> Game {
        Game {
            grid,
            swap: Default::default(),
        }
    }

    pub fn run(&mut self) {
        println!("{}", self.grid);
        while !self.grid.is_empty() {
            self.tick();
            println!("{}", self.grid);
            thread::sleep(Duration::from_millis(1000));
        }
    }

    pub fn tick(&mut self) {
        for cell in self.grid.iter() {
            if self.survives(cell) {
                self.swap.set_alive(*cell);
            }
        }
        self.grid = Default::default();
        mem::swap(&mut self.grid, &mut self.swap);
    }

    pub fn survives(&self, cell: &Cell) -> bool {
        let neighbors = self.grid.live_neighbors(cell);
        if self.grid.is_alive(cell) {
            match neighbors.len() {
                2 | 3 => true,
                _ => false,
            }
        } else {
            match neighbors.len() {
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
        let game = Game::new(Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)]));
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
