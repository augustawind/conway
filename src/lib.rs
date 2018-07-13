#[macro_use]
#[cfg(test)]
extern crate maplit;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(pub i64, pub i64);

#[derive(Debug, Default)]
pub struct Grid {
    cells: HashSet<Cell>,
}

impl Grid {
    pub fn new(cells: Vec<Cell>) -> Grid {
        Grid {
            cells: cells.into_iter().collect(),
        }
    }

    pub fn is_alive(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    pub fn is_dead(&self, cell: &Cell) -> bool {
        !self.is_alive(cell)
    }

    pub fn set_alive(&mut self, cell: Cell) -> bool {
        self.cells.insert(cell)
    }

    pub fn set_dead(&mut self, cell: &Cell) -> bool {
        self.cells.remove(cell)
    }

    pub fn live_neighbors(&self, cell: &Cell) -> HashSet<Cell> {
        let Cell(x, y) = cell;
        let mut neighbors = HashSet::new();
        let mut neighbor: Cell;
        for dx in -1..=1 {
            for dy in -1..=1 {
                neighbor = Cell(x + dx, y + dy);
                if &neighbor != cell && self.is_alive(&neighbor) {
                    neighbors.insert(neighbor);
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn test_is_alive() {
        let grid = Grid::new(vec![Cell(-1, 4), Cell(8, 8)]);
        assert!(&grid.is_alive(&Cell(-1, 4)));
        assert!(&grid.is_alive(&Cell(8, 8)));
        assert!(!&grid.is_alive(&Cell(8, 4)));
    }

    #[test]
    fn test_set_alive() {
        let mut grid: Grid = Default::default();
        let cell = Cell(3, -3);
        assert!(!&grid.is_alive(&cell));
        grid.set_alive(cell);
        assert!(&grid.is_alive(&cell));
        grid.set_dead(&cell);
        assert!(!&grid.is_alive(&cell));
    }

    #[test]
    fn test_live_neighbors() {
        let grid: Grid = Grid::new(vec![Cell(-1, -1), Cell(-1, -2), Cell(0, 0), Cell(1, 0)]);
        assert_eq!(
            grid.live_neighbors(&Cell(0, 0)),
            hashset![Cell(-1, -1), Cell(1, 0)],
            "it should work for a live cell"
        );
        assert_eq!(
            grid.live_neighbors(&Cell(-1, -3)),
            hashset![Cell(-1, -2)],
            "it should work for a dead cell"
        )
    }
}
