extern crate conway;

use conway::grid::{Cell, Grid};

fn main() {
    let grid = Grid::new(vec![Cell(0, 0), Cell(0, 1), Cell(1, 1), Cell(2, 1)]);
    println!("{:#?}", grid);
}
