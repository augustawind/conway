use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use num_integer::Integer;

use super::error::AppError;

const CHAR_ALIVE: char = 'x';
const CHAR_DEAD: char = '.';

/// A Cell is a point on the `Grid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(pub i64, pub i64);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Cell(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

/// A Grid represents the physical world in which Conway's Game of Life takes place.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Grid {
    cells: HashSet<Cell>,
    min_width: u64,
    min_height: u64,
    pub char_alive: char,
    pub char_dead: char,
}

impl Grid {
    /// Create a new Grid.
    pub fn new(
        cells: Vec<Cell>,
        min_width: u64,
        min_height: u64,
        char_alive: char,
        char_dead: char,
    ) -> Grid {
        let mut grid = Grid {
            cells: cells.into_iter().collect(),
            min_width,
            min_height,
            char_alive,
            char_dead,
        };

        // min_width and min_height will be at least the starting Grid's natural size.
        let (width, height) = grid.natural_size();
        grid.min_width = cmp::max(min_width, width);
        grid.min_height = cmp::max(min_height, height);
        grid
    }

    /// Return the coordinates of a "viewport" surrounding the Grid's activity.
    pub fn viewport(&self) -> ((i64, i64), (i64, i64)) {
        let (width, height) = self.natural_size();
        let (dx, dy) = (
            cmp::max(0, self.min_width - width) as i64,
            cmp::max(0, self.min_height - height) as i64,
        );

        let ((x0, y0), (x1, y1)) = self.calculate_bounds();
        let ((dx0, dx1), (dy0, dy1)) = (split_int(dx), split_int(dy));

        ((x0 - dx0, y0 - dy0), (x1 + dx1, y1 + dy1))
    }

    /// Return the Grid's minimum width and height in a tuple `(min_width, min_height)`.
    pub fn min_size(&self) -> (u64, u64) {
        (self.min_width, self.min_height)
    }

    /// Return whether the Grid is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Return whether the given Cell is alive.
    pub fn is_alive(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    /// Bring the given Cell to life.
    pub fn set_alive(&mut self, cell: Cell) -> bool {
        self.cells.insert(cell)
    }

    /// Kill the given Cell.
    pub fn set_dead(&mut self, cell: &Cell) -> bool {
        self.cells.remove(cell)
    }

    /// Clear the Grid of all living Cells.
    pub fn clear(&mut self) {
        self.cells.clear()
    }

    /// Return the set of all Cells in the Grid that should be evaluated for survival.
    pub fn active_cells(&self) -> HashSet<Cell> {
        self.cells
            .iter()
            .flat_map(|cell| {
                let mut cells = self.adjacent_cells(cell);
                cells.insert(*cell);
                cells
            })
            .collect()
    }

    /// Return all 8 Cells that are directly adjacent to the given Cell.
    pub fn adjacent_cells(&self, cell: &Cell) -> HashSet<Cell> {
        let Cell(x, y) = cell;
        let mut cells = HashSet::with_capacity(8);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                cells.insert(Cell(x + dx, y + dy));
            }
        }
        cells
    }

    /// Return the number of living Cells that are adjacent to the given Cell.
    pub fn live_neighbors(&self, cell: &Cell) -> usize {
        self.adjacent_cells(cell)
            .iter()
            .filter(|c| self.is_alive(c))
            .count()
    }

    // Return the Grid's width and height as the X and Y distance between its furthest Cells.
    fn natural_size(&self) -> (u64, u64) {
        let ((x0, y0), (x1, y1)) = self.calculate_bounds();
        ((x1 - x0 + 1) as u64, (y1 - y0 + 1) as u64)
    }

    // Return the lowest and highest X and Y coordinates represented in the Grid.
    fn calculate_bounds(&self) -> ((i64, i64), (i64, i64)) {
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
}

/// Create a visual string representation of the Grid with each character representing a Cell.
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((x0, y0), (x1, y1)) = self.viewport();

        let mut output = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                output.push(if self.is_alive(&Cell(x, y)) {
                    self.char_alive
                } else {
                    self.char_dead
                });
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

/// Parse a Grid from a visual string representation.
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

        Ok(Grid::new(cells, width, height, CHAR_ALIVE, CHAR_DEAD))
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
    fn test_new_grid_size() {
        assert_eq!(
            Grid::new(vec![Cell(0, 0), Cell(5, 5)], 3, 3, 'x', '.').min_size(),
            (6, 6),
            "natural size should override given size if smaller"
        );
        assert_eq!(
            Grid::new(vec![Cell(0, 0), Cell(5, 5)], 8, 8, 'x', '.').min_size(),
            (8, 8),
            "given size should override natural size if larger"
        );
    }

    #[test]
    fn test_is_empty() {
        let grid: Grid = Default::default();
        assert!(grid.is_empty());
        let grid = Grid::new(vec![Cell(0, 0)], 0, 0, 'x', '.');
        assert!(!grid.is_empty());
    }

    #[test]
    fn test_is_alive() {
        let grid = Grid::new(vec![Cell(-1, 4), Cell(8, 8)], 0, 0, 'x', '.');
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
    fn test_viewport() {
        assert_eq!(
            Grid::new(
                vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                7,
                7,
                'x',
                '.'
            ).viewport(),
            ((-3, -2), (3, 4)),
            "should raise the bounds to match min_width and min_height, if smaller"
        );
        assert_eq!(
            Grid::new(
                vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)],
                88,
                32,
                'x',
                '.'
            ).viewport(),
            ((-23, 1), (64, 33)),
            "should not raise the bounds if they are larger than min_width and min_height"
        );
        assert_eq!(
            Grid::new(
                vec![Cell(2, 3), Cell(3, 3), Cell(5, 4), Cell(4, 2)],
                10,
                10,
                'x',
                '.'
            ).viewport(),
            ((-1, -1), (8, 8)),
        );
    }

    #[test]
    fn test_calculate_bounds() {
        assert_eq!(
            Grid::new(
                vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                0,
                0,
                'x',
                '.'
            ).calculate_bounds(),
            ((-3, 0), (2, 1))
        );
        assert_eq!(
            Grid::new(vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)], 0, 0, 'x', '.')
                .calculate_bounds(),
            ((-12, 1), (53, 33))
        );
    }

    #[test]
    fn test_active_cells() {
        let grid = Grid::new(vec![Cell(0, 0), Cell(1, 1)], 0, 0, 'x', '.');
        assert_eq!(
            grid.active_cells(),
            hashset![
                Cell(0, 0),
                Cell(-1, -1),
                Cell(0, -1),
                Cell(1, -1),
                Cell(1, 0),
                Cell(1, 1),
                Cell(0, 1),
                Cell(-1, 1),
                Cell(-1, 0),
                Cell(2, 0),
                Cell(2, 1),
                Cell(2, 2),
                Cell(1, 2),
                Cell(0, 2),
            ]
        )
    }

    #[test]
    fn test_live_neighbors() {
        let grid = Grid::new(
            vec![Cell(-1, -1), Cell(-1, -2), Cell(0, 0), Cell(1, 0)],
            0,
            0,
            'x',
            '.',
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
            Grid::new(
                vec![Cell(0, 0), Cell(1, 0), Cell(2, 1), Cell(1, 2)],
                3,
                3,
                'x',
                '.'
            ),
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
