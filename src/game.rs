use std::cmp;
use std::mem;
use std::str::FromStr;
use std::thread;

use num_integer::Integer;

use config::ConfigSet;
pub use config::GameConfig;
use grid::{Cell, Grid};
use {AppError, AppResult};

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

pub struct GameIter<'a>(&'a mut Game);

impl<'a> Iterator for GameIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_over() {
            return None;
        }
        self.0.tick();
        thread::sleep(self.0.opts.tick_delay);
        Some(self.0.draw())
    }
}

/// Game holds the high-level gameplay logic.
#[derive(Debug)]
pub struct Game {
    grid: Grid,
    swap: Grid,
    opts: GameConfig,
    viewport: Viewport,
}

impl Game {
    pub fn load() -> AppResult<Game> {
        let config = ConfigSet::from_env()?;
        let grid = Grid::from_config(config.grid)?;
        Ok(Game::new(grid, config.game))
    }

    pub fn new(grid: Grid, mut opts: GameConfig) -> Game {
        let mut swap = grid.clone();
        swap.clear();

        let (origin, Cell(x1, y1)) = grid.calculate_bounds();
        let (width, height) = ((x1 - origin.0 + 1) as u64, (y1 - origin.1 + 1) as u64);

        // set min dimensions to at least the starting Grid's natural size
        opts.min_width = cmp::max(opts.min_width, width);
        opts.min_height = cmp::max(opts.min_height, height);
        let viewport = Viewport {
            origin,
            width: if opts.width == 0 {
                width
            } else {
                opts.width
            },
            height: if opts.height == 0 {
                height
            } else {
                opts.height
            },
            scroll: Cell(0, 0),
        };

        Game {
            grid,
            swap,
            opts,
            viewport,
        }
    }

    pub fn iter(&mut self) -> GameIter {
        GameIter(self)
    }

    pub fn draw(&self) -> String {
        self.draw_viewport(self.viewport())
    }

    fn draw_viewport(&self, (Cell(x0, y0), Cell(x1, y1)): (Cell, Cell)) -> String {
        let mut output = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                output.push(if self.grid.is_alive(&Cell(x, y)) {
                    self.opts.char_alive
                } else {
                    self.opts.char_dead
                });
            }
            output.push('\n');
        }
        output
    }

    pub fn scroll(&mut self, dx: i64, dy: i64) {
        self.viewport.scroll = self.viewport.scroll - Cell(dx, dy);
    }

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
        let (Cell(x0, y0), Cell(x1, y1)) = self.grid.calculate_bounds();
        let (width, height) = ((x1 - x0 + 1) as u64, (y1 - y0 + 1) as u64);
        let (dx, dy) = (
            cmp::max(0, self.opts.min_width - width) as i64,
            cmp::max(0, self.opts.min_height - height) as i64,
        );

        let ((dx0, dx1), (dy0, dy1)) = (split_int(dx), split_int(dy));

        (Cell(x0 - dx0, y0 - dy0), Cell(x1 + dx1, y1 + dy1))
    }

    /// Return whether the Game is over. This happens with the Grid is empty.
    pub fn is_over(&self) -> bool {
        self.grid.is_empty()
    }

    /// Execute the next turn in the Game of Life.
    ///
    /// `tick` applies the rules of game to each individual Cell, killing some and reviving others.
    pub fn tick(&mut self) {
        for cell in self.grid.active_cells() {
            if self.survives(&cell) {
                self.swap.set_alive(cell);
            }
        }
        self.grid.clear();
        mem::swap(&mut self.grid, &mut self.swap);
    }

    /// Survives returns whether the given Cell survives an application of the Game Rules.
    pub fn survives(&self, cell: &Cell) -> bool {
        let live_neighbors = self.grid.live_neighbors(cell);
        if self.grid.is_alive(cell) {
            match live_neighbors {
                2 | 3 => true,
                _ => false,
            }
        } else {
            match live_neighbors {
                3 => true,
                _ => false,
            }
        }
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

    #[test]
    fn test_min_size() {
        let game = Game::new(
            Grid::new(vec![Cell(0, 0), Cell(5, 5)], Default::default()),
            GameConfig {
                min_width: 8,
                min_height: 8,
                ..Default::default()
            },
        );
        assert_eq!((game.opts.min_width, game.opts.min_height), (8, 8),);
    }

    #[test]
    fn test_min_size_override() {
        let game = Game::new(
            Grid::new(vec![Cell(0, 0), Cell(5, 5)], Default::default()),
            GameConfig {
                min_width: 3,
                min_height: 3,
                ..Default::default()
            },
        );
        assert_eq!(
            (game.opts.min_width, game.opts.min_height),
            (6, 6),
            "natural size should override given min size if natural > given"
        );
    }

    #[test]
    fn test_survives_blinker() {
        let game = Game::new(
            Grid::new(vec![Cell(1, 0), Cell(1, 1), Cell(1, 2)], Default::default()),
            Default::default(),
        );
        assert!(
            game.survives(&Cell(1, 1)),
            "a live cell with 2 live neighbors should survive"
        );
        assert!(
            game.survives(&Cell(0, 1)),
            "a dead cell with 3 live neighbors should survive"
        );
        assert!(
            game.survives(&Cell(2, 1)),
            "a dead cell with 3 live neighbors should survive"
        );
        assert!(
            !game.survives(&Cell(1, 0)),
            "a live cell with < 2 live neighbors should die"
        );
        assert!(
            !game.survives(&Cell(1, 2)),
            "a live cell with < 2 live neighbors should die"
        );
    }

    mod viewport {
        use super::*;

        #[test]
        fn test_viewport_centered_1() {
            assert_eq!(
                Game::new(
                    Grid::new(
                        vec![Cell(2, 1), Cell(-3, 0), Cell(-2, 1), Cell(-2, 0)],
                        Default::default()
                    ),
                    GameConfig {
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
                Game::new(
                    Grid::new(
                        vec![Cell(53, 4), Cell(2, 1), Cell(-12, 33)],
                        Default::default()
                    ),
                    GameConfig {
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
                Game::new(
                    Grid::new(
                        vec![Cell(2, 3), Cell(3, 3), Cell(5, 4), Cell(4, 2)],
                        Default::default()
                    ),
                    GameConfig {
                        min_width: 10,
                        ..Default::default()
                    }
                ).viewport_centered(),
                (Cell(-1, -1), Cell(8, 8)),
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
