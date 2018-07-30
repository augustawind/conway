use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::num::ParseIntError;
use std::ops;
use std::path::Path;
use std::str::FromStr;

use num_integer::Integer;

use config::GridConfig;
use {AppError, AppResult};

/// A Cell is a point on the `Grid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(pub i64, pub i64);

impl ops::Add for Cell {
    type Output = Self;

    fn add(self, rhs: Cell) -> Self::Output {
        Cell(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Cell {
    type Output = Self;

    fn sub(self, rhs: Cell) -> Self::Output {
        Cell(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell(0, 0)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Cell(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

impl FromStr for Cell {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lparen, rest) = s.split_at(1);
        if lparen != "(" {
            return Err(AppError::ParseCell(format!(
                "unexpected character '{}'",
                lparen
            )));
        }
        let (rest, rparen) = rest.split_at(rest.len() - 1);
        if rparen != ")" {
            return Err(AppError::ParseCell(format!(
                "unexpected character '{}'",
                rparen
            )));
        }
        let mut nums = rest.split(',');
        let x: i64 = nums
            .next()
            .ok_or_else(|| AppError::ParseCell(format!("missing value for x")))?
            .parse()
            .map_err(|e: ParseIntError| AppError::ParseCell(e.to_string()))?;
        let y: i64 = nums
            .next()
            .ok_or_else(|| AppError::ParseCell(format!("missing value for y")))?
            .parse()
            .map_err(|e: ParseIntError| AppError::ParseCell(e.to_string()))?;
        Ok(Cell(x, y))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Centered,
    Fixed,
    Follow,
}

impl FromStr for View {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "centered" => Ok(View::Centered),
            "fixed" => Ok(View::Fixed),
            "follow" => Ok(View::Follow),
            s => Err(From::from(format!("'{}' is not a valid choice", s))),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Viewport {
    origin: Cell,
    scroll: Cell,
    width: u64,
    height: u64,
}

impl Viewport {
    pub fn new(width: u64, height: u64) -> Self {
        Viewport {
            width,
            height,
            ..Default::default()
        }
    }
}

/// A Grid represents the physical world in which Conway's Game of Life takes place.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Grid {
    cells: HashSet<Cell>,
    opts: GridConfig,
    viewport: Viewport,
}

impl Grid {
    /*
     * Constructors
     */

    /// Create a new Grid.
    pub fn new(cells: Vec<Cell>, opts: GridConfig) -> Self {
        let mut grid = Grid {
            cells: cells.into_iter().collect(),
            viewport: Viewport::new(opts.width, opts.height),
            opts,
        };

        let (origin, Cell(x1, y1)) = grid.calculate_bounds();
        let (width, height) = ((x1 - origin.0 + 1) as u64, (y1 - origin.1 + 1) as u64);
        grid.viewport.origin = origin;
        if grid.viewport.width == 0 {
            grid.viewport.width = width;
        }
        if grid.viewport.height == 0 {
            grid.viewport.height = height;
        }

        // set min dimensions to at least the starting Grid's natural size
        grid.opts.min_width = cmp::max(grid.opts.min_width, width);
        grid.opts.min_height = cmp::max(grid.opts.min_height, height);

        grid
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        let config = GridConfig {
            pattern: GridConfig::read_pattern(path)?,
            ..Default::default()
        };
        Grid::from_config(config)
    }

    pub fn from_config(config: GridConfig) -> AppResult<Self> {
        let mut cells = Vec::new();

        for (y, line) in config
            .pattern
            .trim()
            .lines()
            .filter(|line| !line.starts_with('#'))
            .enumerate()
        {
            for (x, ch) in line.chars().enumerate() {
                // Living Cells are added to the Grid.
                if ch == config.char_alive {
                    cells.push(Cell(x as i64, y as i64));
                // Dead Cells are ignored, and any other symbol is an error.
                } else if ch != config.char_dead {
                    return Err(From::from(format!("unknown character: '{}'", ch)));
                }
            }
        }

        Ok(Grid::new(cells, config))
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
     * Viewport
     */

    pub fn viewport(&self) -> (Cell, Cell) {
        match &self.opts.view {
            View::Fixed => self.viewport_fixed(),
            View::Centered => self.viewport_centered(),
            _ => unimplemented!(),
        }
    }

    pub fn viewport_fixed(&self) -> (Cell, Cell) {
        let Cell(x0, y0) = self.viewport.origin + self.viewport.scroll;
        let p1 = Cell(
            x0 + self.viewport.width as i64,
            y0 + self.viewport.height as i64,
        );
        (Cell(x0, y0), p1)
    }

    pub fn viewport_centered(&self) -> (Cell, Cell) {
        let (width, height) = self.natural_size();
        let (dx, dy) = (
            cmp::max(0, self.opts.min_width - width) as i64,
            cmp::max(0, self.opts.min_height - height) as i64,
        );

        let (Cell(x0, y0), Cell(x1, y1)) = self.calculate_bounds();
        let ((dx0, dx1), (dy0, dy1)) = (split_int(dx), split_int(dy));

        (Cell(x0 - dx0, y0 - dy0), Cell(x1 + dx1, y1 + dy1))
    }

    pub fn scroll(&mut self, dx: i64, dy: i64) {
        self.viewport.scroll = self.viewport.scroll + Cell(dx, dy);
    }

    /*
     * Geometry
     */

    // Return the Grid's width and height as the X and Y distance between its furthest Cells.
    fn natural_size(&self) -> (u64, u64) {
        let (Cell(x0, y0), Cell(x1, y1)) = self.calculate_bounds();
        ((x1 - x0 + 1) as u64, (y1 - y0 + 1) as u64)
    }

    // Return the lowest and highest X and Y coordinates represented in the Grid.
    fn calculate_bounds(&self) -> (Cell, Cell) {
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

/// Create a visual string representation of the Grid with each character representing a Cell.
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (Cell(x0, y0), Cell(x1, y1)) = self.viewport();

        let mut coords = Vec::new();
        let mut output = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                output.push(if self.is_alive(&Cell(x, y)) {
                    coords.push(Cell(x, y).to_string());
                    self.opts.char_alive
                } else {
                    self.opts.char_dead
                });
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

/// Parse a Grid from a block of structured text.
///
/// Since `from_str` takes no parameters, a default GridConfig is used.
/// To use your own GridConfig, use `from_config` instead.
impl FromStr for Grid {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Grid::from_config(GridConfig {
            pattern: s.to_owned(),
            ..Default::default()
        })
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
    use config::GridConfig;
    use std::default::Default;

    mod constructors {
        use super::*;

        #[test]
        fn test_grid_min_size() {
            let grid = Grid::new(
                vec![Cell(0, 0), Cell(5, 5)],
                GridConfig {
                    min_width: 8,
                    min_height: 8,
                    ..Default::default()
                },
            );
            assert_eq!((grid.opts.min_width, grid.opts.min_height), (8, 8),);
        }

        #[test]
        fn test_grid_min_size_override() {
            let grid = Grid::new(
                vec![Cell(0, 0), Cell(5, 5)],
                GridConfig {
                    min_width: 3,
                    min_height: 3,
                    ..Default::default()
                },
            );
            assert_eq!(
                (grid.opts.min_width, grid.opts.min_height),
                (6, 6),
                "natural size should override given min size if natural > given"
            );
        }

        #[test]
        fn test_from_config() {
            let (char_alive, char_dead) = ('!', '_');
            let pattern = vec![
                format!("{}{}", char_alive, char_alive),
                format!("{}{}{}", char_dead, char_dead, char_alive),
                format!("{}{}{}", char_dead, char_alive, char_dead),
            ].join("\n");
            let config = GridConfig {
                pattern,
                char_alive,
                char_dead,
                view: View::Centered,
                min_width: 5,
                min_height: 5,
                width: 8,
                height: 8,
            };
            let grid = Grid::from_config(config.clone()).unwrap();

            assert_eq!(
                grid,
                Grid::new(vec![Cell(0, 0), Cell(1, 0), Cell(2, 1), Cell(1, 2)], config),
            );
        }

        #[test]
        fn test_from_str() {
            use config::{CHAR_ALIVE, CHAR_DEAD};
            let grid: Grid = vec![
                format!("{}{}", CHAR_ALIVE, CHAR_ALIVE),
                format!("{}{}{}", CHAR_DEAD, CHAR_DEAD, CHAR_ALIVE),
                format!("{}{}{}", CHAR_DEAD, CHAR_ALIVE, CHAR_DEAD),
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
            let grid = Grid::new(vec![Cell(0, 0), Cell(1, 1)], Default::default());
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
                Default::default(),
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
        fn test_is_empty() {
            let grid: Grid = Default::default();
            assert!(grid.is_empty());
            let grid = Grid::new(vec![Cell(0, 0)], Default::default());
            assert!(!grid.is_empty());
        }

        #[test]
        fn test_is_alive() {
            let grid = Grid::new(vec![Cell(-1, 4), Cell(8, 8)], Default::default());
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

    mod viewport {
        use super::*;

        #[test]
        fn test_viewport_centered_1() {
            assert_eq!(
                Grid::new(
                    vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                    GridConfig {
                        min_width: 7,
                        min_height: 7,
                        ..Default::default()
                    }
                ).viewport_centered(),
                (Cell(-3, -2), Cell(3, 4)),
                "should raise the bounds to match min_width and min_height, if smaller"
            );
        }

        #[test]
        fn test_viewport_centered_2() {
            assert_eq!(
                Grid::new(
                    vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)],
                    GridConfig {
                        min_width: 88,
                        ..Default::default()
                    }
                ).viewport_centered(),
                (Cell(-23, 1), Cell(64, 33)),
                "should not raise the bounds if they are larger than min_width and min_height"
            );
        }

        #[test]
        fn test_viewport_centered_3() {
            assert_eq!(
                Grid::new(
                    vec![Cell(2, 3), Cell(3, 3), Cell(5, 4), Cell(4, 2)],
                    GridConfig {
                        min_width: 10,
                        ..Default::default()
                    }
                ).viewport_centered(),
                (Cell(-1, -1), Cell(8, 8)),
            );
        }
    }

    mod geometry {
        use super::*;

        #[test]
        fn test_calculate_bounds_1() {
            assert_eq!(
                Grid::new(
                    vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                    Default::default()
                ).calculate_bounds(),
                (Cell(-3, 0), Cell(2, 1))
            );
        }

        #[test]
        fn test_calculate_bounds_2() {
            assert_eq!(
                Grid::new(
                    vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)],
                    Default::default()
                ).calculate_bounds(),
                (Cell(-12, 1), Cell(53, 33))
            );
        }
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
