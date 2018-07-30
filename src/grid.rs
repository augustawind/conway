use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

pub use cell::Cell;
pub use config::GridConfig;
use {AppError, AppResult};

pub const READ_CHAR_ALIVE: char = 'x';
pub const READ_CHAR_DEAD: char = '.';

/// A Grid represents the physical world in which Conway's Game of Life takes place.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Grid {
    cells: HashSet<Cell>,
}

impl Grid {
    /*
     * Constructors
     */

    /// Create a new Grid.
    pub fn new(cells: Vec<Cell>) -> Self {
        Grid {
            cells: cells.into_iter().collect(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        let pattern = GridConfig::read_pattern(path)?;
        pattern.parse()
    }

    /*
     * Cells
     */

    /// Return the number of living Cells that are adjacent to the given Cell.
    pub fn live_neighbors(&self, cell: &Cell) -> usize {
        self.adjacent_cells(cell)
            .iter()
            .filter(|c| self.is_alive(c))
            .count()
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

    /*
     * Geometry
     */

    // Return the lowest and highest X and Y coordinates represented in the Grid.
    pub fn calculate_bounds(&self) -> (Cell, Cell) {
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
            (Cell(x0, y0), Cell(x1, y1))
        } else {
            (Default::default(), Default::default())
        }
    }
}

/// Parse a Grid from a block of structured text.
///
/// Since `from_str` takes no parameters, a default GridConfig is used.
/// To use your own GridConfig, use `from_config` instead.
impl FromStr for Grid {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();

        for (y, line) in s
            .trim()
            .lines()
            .filter(|line| !line.starts_with('#'))
            .enumerate()
        {
            for (x, ch) in line.chars().enumerate() {
                // Living Cells are added to the Grid.
                if ch == READ_CHAR_ALIVE {
                    cells.push(Cell(x as i64, y as i64));
                // Dead Cells are ignored, and any other symbol is an error.
                } else if ch != READ_CHAR_DEAD {
                    return Err(From::from(format!("unknown character: '{}'", ch)));
                }
            }
        }

        Ok(Grid::new(cells))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::default::Default;

    mod constructors {
        use super::*;

        #[test]
        fn test_from_str() {
            let grid: Grid = vec![
                format!("{}{}", READ_CHAR_ALIVE, READ_CHAR_ALIVE),
                format!("{}{}{}", READ_CHAR_DEAD, READ_CHAR_DEAD, READ_CHAR_ALIVE),
                format!("{}{}{}", READ_CHAR_DEAD, READ_CHAR_ALIVE, READ_CHAR_DEAD),
            ].join("\n")
                .parse()
                .unwrap();

            assert_eq!(
                grid.cells,
                hashset![Cell(0, 0), Cell(1, 0), Cell(2, 1), Cell(1, 2)],
            );
            assert!(Grid::from_str("abc\ndef").is_err())
        }
    }

    mod cells {
        use super::*;

        #[test]
        fn test_active_cells() {
            let grid = Grid::new(vec![Cell(0, 0), Cell(1, 1)]);
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
            let grid = Grid::new(vec![Cell(-1, -1), Cell(-1, -2), Cell(0, 0), Cell(1, 0)]);
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
        fn test_is_empty() {
            let grid: Grid = Default::default();
            assert!(grid.is_empty());
            let grid = Grid::new(vec![Cell(0, 0)]);
            assert!(!grid.is_empty());
        }

        #[test]
        fn test_is_alive() {
            let grid = Grid::new(vec![Cell(-1, 4), Cell(8, 8)]);
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
    }

    mod geometry {
        use super::*;

        #[test]
        fn test_calculate_bounds_1() {
            assert_eq!(
                Grid::new(vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],)
                    .calculate_bounds(),
                (Cell(-3, 0), Cell(2, 1))
            );
        }

        #[test]
        fn test_calculate_bounds_2() {
            assert_eq!(
                Grid::new(vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)],).calculate_bounds(),
                (Cell(-12, 1), Cell(53, 33))
            );
        }
    }
}
