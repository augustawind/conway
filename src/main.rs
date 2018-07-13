extern crate conway;

use conway::{Cell, Game, Grid};

fn main() {
    let grid = Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)]);
    println!("{:#?}", &grid);
    let mut game = Game::new(grid, 10, 10);
    game.run();
}
