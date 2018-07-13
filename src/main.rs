extern crate clap;
extern crate conway;

use conway::{Cell, Game, Grid};

fn main() {
    let grid = Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)], 10, 10);
    let game = Game::new(grid);
    println!("{:#?}", game);
}
