use std::mem;

use super::grid::{Cell, Grid};

#[derive(Debug)]
pub struct Game {
    pub grid: Grid,
    swap: Grid,
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

impl Game {
    pub fn new(grid: Grid) -> Game {
        let swap = Grid::new(Vec::new(), grid.min_width, grid.min_height);
        Game { grid, swap }
    }

    pub fn draw(&self) -> String {
        self.grid.to_string()
    }

    pub fn tick(&mut self) {
        for cell in self.grid.all_cells() {
            if self.survives(&cell) {
                self.swap.set_alive(cell);
            }
        }
        self.grid.clear();
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
