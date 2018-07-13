use std::cmp;
use std::default::Default;
use std::mem;
use std::thread;
use std::time::Duration;

use super::grid::{Cell, Grid};

pub struct Game {
    grid: Grid,
    swap: Grid,
    min_width: u64,
    min_height: u64,
}

impl Game {
    pub fn new(grid: Grid, min_width: u64, min_height: u64) -> Game {
        Game {
            grid,
            min_width,
            min_height,
            swap: Default::default(),
        }
    }

    pub fn draw(&self) -> String {
        let ((mut x0, mut y0), (mut x1, mut y1)) = self.grid.calculate_bounds();
        x1 -= x0;
        x0 = 0;
        y1 -= y0;
        y0 = 0;
        let (min_x, min_y) = (self.min_width as i64 - 1, self.min_height as i64 - 1);
        x1 = cmp::max(x1, min_x);
        y1 = cmp::max(y1, min_y);

        let mut output = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                output.push(if self.grid.is_alive(&Cell(x, y)) {
                    'x'
                } else {
                    '.'
                });
            }
            output.push('\n');
        }
        output
    }

    pub fn run(&mut self) {
        println!("{}", self.draw());
        // println!("SWAP\n{}\n", self.swap);
        while !self.grid.is_empty() {
            self.tick();
            println!("{}", self.draw());
            // println!("SWAP\n{}\n", self.swap);
            thread::sleep(Duration::from_millis(1000));
        }
    }

    pub fn tick(&mut self) {
        for cell in self.grid.iter() {
            if self.survives(cell) {
                self.swap.set_alive(*cell);
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
        let game = Game::new(Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)]), 0, 0);
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
