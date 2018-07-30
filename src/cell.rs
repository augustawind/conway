use std::fmt;
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;

use super::AppError;

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
        let (lparen, rest) = s.trim().split_at(1);
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
            .trim()
            .parse()
            .map_err(|e: ParseIntError| AppError::ParseCell(e.to_string()))?;
        let y: i64 = nums
            .next()
            .ok_or_else(|| AppError::ParseCell(format!("missing value for y")))?
            .trim()
            .parse()
            .map_err(|e: ParseIntError| AppError::ParseCell(e.to_string()))?;
        Ok(Cell(x, y))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Cell(1, 1) + Cell(4, 9), Cell(5, 10));
        assert_eq!(Cell(-3, 5) + Cell(-5, -6), Cell(-8, -1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Cell(1, 1) - Cell(4, 9), Cell(-3, -8));
        assert_eq!(Cell(-3, 5) - Cell(-5, -6), Cell(2, 11));
    }

    #[test]
    fn test_from_str() {
        assert_eq!("(-4, 9)".parse::<Cell>().unwrap(), Cell(-4, 9));
    }
}
