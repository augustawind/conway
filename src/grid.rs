use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use num_integer::Integer;

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
    min_width: u64,
    min_height: u64,
}

impl Grid {
    pub fn new(cells: Vec<Cell>, min_width: u64, min_height: u64) -> Grid {
        let mut grid = Grid {
            cells: cells.into_iter().collect(),
            min_width,
            min_height,
        };
        let (width, height) = grid.natural_size();
        grid.min_width = cmp::max(min_width, width);
        grid.min_height = cmp::max(min_height, height);
        grid
    }

    fn natural_size(&self) -> (u64, u64) {
        let ((x0, y0), (x1, y1)) = self.calculate_bounds_raw();
        ((x1 - x0 + 1) as u64, (y1 - y0 + 1) as u64)
    }

    pub fn calculate_bounds(&self) -> ((i64, i64), (i64, i64)) {
        let (width, height) = self.natural_size();
        let (dx, dy) = (
            cmp::max(0, self.min_width - width) as i64,
            cmp::max(0, self.min_height - height) as i64,
        );

        let ((x0, y0), (x1, y1)) = self.calculate_bounds_raw();
        let ((dx0, dx1), (dy0, dy1)) = (split_int(dx), split_int(dy));

        ((x0 - dx0, y0 - dy0), (x1 + dx1, y1 + dy1))
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

    pub fn live_neighbors(&self, cell: &Cell) -> usize {
        self.adjacent_cells(cell)
            .iter()
            .filter(|c| self.is_alive(c))
            .count()
    }

    pub fn adjacent_cells(&self, cell: &Cell) -> Vec<Cell> {
        let Cell(x, y) = cell;
        let mut adj_cells = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                adj_cells.push(Cell(x + dx, y + dy));
            }
        }
        adj_cells
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

        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in s.trim().lines().enumerate() {
            height += 1;
            width = cmp::max(width, line.len() as u64);
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    CHAR_ALIVE => cells.push(Cell(x as i64, y as i64)),
                    CHAR_DEAD => (),
                    _ => return Err(AppError(format!("unknown character: '{}'", ch))),
                }
            }
        }

        Ok(Grid::new(cells, width, height))
    }
}

fn split_int<T: Integer + Copy>(n: T) -> (T, T) {
    let two = T::one() + T::one();
    let (quotient, remainder) = n.div_rem(&two);
    (quotient, quotient + remainder)
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
            2,
            "it should work for a live cell"
        );
        assert_eq!(
            grid.live_neighbors(&Cell(-1, -3)),
            1,
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
                7,
                7
            ).calculate_bounds(),
            ((-3, -2), (3, 4)),
            "should raise the bounds to match min_width and min_height, if smaller"
        );
        assert_eq!(
            Grid::new(vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)], 88, 32).calculate_bounds(),
            ((-23, 1), (64, 33)),
            "should not raise the bounds if they are larger than min_width and min_height"
        );
        assert_eq!(
            Grid::new(vec![Cell(2, 3), Cell(3, 3), Cell(5, 4), Cell(4, 2)], 10, 10)
                .calculate_bounds(),
            ((-1, -1), (8, 8)),
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

    #[test]
    fn test_split_int() {
        assert_eq!(split_int(30), (15, 15));
        assert_eq!(split_int(31), (15, 16));
        assert_eq!(split_int(32), (16, 16));
        assert_eq!(split_int(0), (0, 0));
        assert_eq!(split_int(1), (0, 1));
        assert_eq!(split_int(2), (1, 1));
    }
}
