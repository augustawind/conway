use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use super::error::AppError;

const CHAR_ALIVE: char = 'x';
const CHAR_DEAD: char = '.';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(pub i64, pub i64);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Cell(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Grid {
    cells: HashSet<Cell>,
    pub min_width: u64,
    pub min_height: u64,
}

impl Grid {
    pub fn new(cells: Vec<Cell>, min_width: u64, min_height: u64) -> Grid {
        Grid {
            cells: cells.into_iter().collect(),
            min_width,
            min_height,
        }
    }

    pub fn all_cells(&self) -> Vec<Cell> {
        let ((x0, y0), (x1, y1)) = self.calculate_bounds();
        let mut result = Vec::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                result.push(Cell(x, y));
            }
        }
        result
    }

    pub fn calculate_bounds(&self) -> ((i64, i64), (i64, i64)) {
        let ((x0, y0), (x1, y1)) = self.calculate_bounds_raw();
        let (width, height) = (x1 - x0, y1 - y0);
        let (dx, dy) = (
            cmp::max(0, self.min_width as i64 - width),
            cmp::max(0, self.min_height as i64 - height),
        );
        ((x0, y0), (x1 + dx, y1 + dy))
    }

    fn calculate_bounds_raw(&self) -> ((i64, i64), (i64, i64)) {
        let mut cells = self.cells.iter();
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

    // TODO: can this simply return a count?
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((x0, y0), (x1, y1)) = self.calculate_bounds();

        let mut output = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                output.push(if self.is_alive(&Cell(x, y)) {
                    CHAR_ALIVE
                } else {
                    CHAR_DEAD
                });
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl FromStr for Grid {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(AppError("string cannot be empty".to_string()));
        }

        let lines: Vec<&str> = s.trim().lines().collect();
        let height = lines.len();
        let mut width = lines[0].len();

        let mut cells = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            width = cmp::max(width, line.len());
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    CHAR_ALIVE => cells.push(Cell(x as i64, y as i64)),
                    CHAR_DEAD => (),
                    _ => return Err(AppError(format!("unknown character: '{}'", ch))),
                }
            }
        }

        Ok(Grid::new(cells, width as u64, height as u64))
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
    fn test_calculate_bounds_raw() {
        assert_eq!(
            Grid::new(
                vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                0,
                0
            ).calculate_bounds_raw(),
            ((-3, 0), (2, 1))
        );
        assert_eq!(
            Grid::new(vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)], 0, 0).calculate_bounds_raw(),
            ((-12, 1), (53, 33))
        );
    }

    #[test]
    fn test_calculate_bounds() {
        assert_eq!(
            Grid::new(
                vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                6,
                6
            ).calculate_bounds(),
            ((-3, 0), (3, 6)),
            "should raise the upper bounds to min_width and min_height, if smaller"
        );
        assert_eq!(
            Grid::new(vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)], 88, 32).calculate_bounds(),
            ((-12, 1), (76, 33)),
            "should not raise the upper bounds if they are larger than min_width and min_height"
        );
        assert_eq!(
            Grid::new(vec![Cell(2, 3), Cell(3, 3), Cell(5, 4), Cell(4, 2)], 10, 10)
                .calculate_bounds(),
            ((2, 2), (12, 12)),
        );
    }

    #[test]
    fn test_from_str() {
        let grid: Grid = vec![
            format!("{}{}", CHAR_ALIVE, CHAR_ALIVE),
            format!("{}{}{}", CHAR_DEAD, CHAR_DEAD, CHAR_ALIVE),
            format!("{}{}{}", CHAR_DEAD, CHAR_ALIVE, CHAR_DEAD),
        ].join("\n")
            .parse()
            .unwrap();
        assert_eq!(
            grid,
            Grid::new(vec![Cell(0, 0), Cell(1, 0), Cell(2, 1), Cell(1, 2)], 3, 3),
        );

        assert!(Grid::from_str("").is_err());
        assert!(Grid::from_str("abc\ndef").is_err())
    }
}
