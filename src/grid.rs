use std::cmp;
use std::collections::hash_set;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(pub i64, pub i64);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Cell(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grid {
    cells: HashSet<Cell>,
    pub min_width: u64,
    pub min_height: u64,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((x1, y1), (dx, dy)) = self.calculate_bounds();

        let mut output = String::new();
        for y in 0..=y1 {
            for x in 0..=x1 {
                output.push(if self.is_alive(&Cell(x + dx, y + dy)) {
                    'x'
                } else {
                    '.'
                });
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl Grid {
    pub fn new(cells: Vec<Cell>, min_width: u64, min_height: u64) -> Grid {
        Grid {
            cells: cells.into_iter().collect(),
            min_width,
            min_height,
        }
    }

    pub fn as_vector(&self) -> Vec<Cell> {
        let ((x1, y1), _) = self.calculate_bounds();
        let mut result = Vec::new();
        for y in 0..=y1 {
            for x in 0..=x1 {
                result.push(Cell(x, y));
            }
        }
        result
    }

    pub fn calculate_bounds(&self) -> ((i64, i64), (i64, i64)) {
        let ((x0, y0), (mut x1, mut y1)) = self.calculate_bounds_raw();
        let (min_x, min_y) = (self.min_width as i64 - 1, self.min_height as i64 - 1);
        let (dx, dy) = (-x0, -y0);
        x1 = cmp::max(x1 + dx, min_x);
        y1 = cmp::max(y1 + dy, min_y);
        ((x1, y1), (dx, dy))
    }

    fn calculate_bounds_raw(&self) -> ((i64, i64), (i64, i64)) {
        let mut cells = self.iter();
        if let Some(&Cell(x, y)) = cells.next() {
            let ((mut x0, mut y0), (mut x1, mut y1)) = ((x, y), (x, y));
            for &Cell(x, y) in cells {
                if x < x0 {
                    x0 = x;
                } else if x > x1 {
                    x1 = x;
                }
                if y < y0 {
                    y0 = y;
                } else if y > y1 {
                    y1 = y;
                }
            }
            ((x0, y0), (x1, y1))
        } else {
            ((0, 0), (0, 0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn is_alive(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    pub fn set_alive(&mut self, cell: Cell) -> bool {
        self.cells.insert(cell)
    }

    pub fn set_dead(&mut self, cell: &Cell) -> bool {
        self.cells.remove(cell)
    }

    pub fn clear(&mut self) {
        self.cells.clear()
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

    pub fn iter(&self) -> hash_set::Iter<Cell> {
        self.cells.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::default::Default;

    #[test]
    fn test_is_empty() {
        let grid: Grid = Default::default();
        assert!(grid.is_empty());
        let grid = Grid::new(vec![Cell(0, 0)], 0, 0);
        assert!(!grid.is_empty());
    }

    #[test]
    fn test_is_alive() {
        let grid = Grid::new(vec![Cell(-1, 4), Cell(8, 8)], 0, 0);
        assert!(&grid.is_alive(&Cell(-1, 4)));
        assert!(&grid.is_alive(&Cell(8, 8)));
        assert!(!&grid.is_alive(&Cell(8, 4)));
    }

    #[test]
    fn test_set_alive_or_dead() {
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
        let grid = Grid::new(
            vec![Cell(-1, -1), Cell(-1, -2), Cell(0, 0), Cell(1, 0)],
            0,
            0,
        );
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

    #[test]
    fn test_calculate_bounds() {
        assert_eq!(
            Grid::new(
                vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                0,
                0
            ).calculate_bounds(),
            ((-3, 0), (2, 1))
        );
        assert_eq!(
            Grid::new(vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)], 0, 0).calculate_bounds(),
            ((-12, 1), (53, 33))
        );
    }
}
