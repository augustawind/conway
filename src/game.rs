use std::default::Default;
use std::mem;
use std::thread;

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
            thread::sleep_ms(1000);
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
