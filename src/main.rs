fn main() {
    let i: Square = "b2".parse().unwrap();
    println!("Result: {}", i);
}

/*
internal representation of column is integer,
can be transformed to char for display purposes
*/
#[derive(PartialEq, Debug)]
pub struct Square {
    column: i8,
    row: i8,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Square: {}, {}", self.column, self.row)
    }
}

pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Square {
    pub fn new(column: i8, row: i8) -> Square {
        match (column, row) {
            (0...7, 0...7) => Square {
                column: column,
                row: row,
            },
            _ => panic!("Row and column must have values between 0 and 7"),
        }
    }

    pub fn from_tuple(tuple: (i8, i8)) -> Square {
        Self::new(tuple.0, tuple.1)
    }

    pub fn neighbour(&self, d: Direction) -> Option<Square> {
        let (d_column, d_row): (i8, i8) = match d {
            Direction::N => (0, 1),
            Direction::NE => (1, 1),
            Direction::E => (1, 0),
            Direction::SE => (1, -1),
            Direction::S => (0, -1),
            Direction::SW => (-1, -1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, 1),
        };
        let (column, row) = (self.column + d_column, self.row + d_row);
        match (column, row) {
            (0...7, 0...7) => Some(Square {
                row: row,
                column: column,
            }),
            _ => None,
        }
    }
}
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::new(err.description())
    }
}

impl FromStr for Square {
    type Err = MyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 2 {
            return Err(MyError::new("x"));
        }
        let mut chars = s.chars();
        chars
            .next()
            .and_then(|c| match c {
                'a'...'h' => Some(c as i8 - 'a' as i8),
                'A'...'H' => Some(c as i8 - 'A' as i8),
                _ => None,
            })
            .and_then(|c| chars.next().map(|r| (c, r)))
            .and_then(|(c, r)| r.to_digit(10).map(|d| (c, d as i8 - 1)))
            .map(Square::from_tuple)
            .ok_or_else(|| MyError::new("y"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g7_has_correct_north_east_neighbour() {
        let square: Square = "G7".parse().unwrap();
        let neighbour = square.neighbour(Direction::NE);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Square::new(7, 7));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn h8_has_no_north_east_neighbour() {
        let square = Square::new(7, 7);
        let neighbour = square.neighbour(Direction::NE);
        assert!(neighbour.is_none());
    }

    #[test]
    fn a1_has_no_south_west_neighbour() {
        let square = Square::new(0, 0);
        let neighbour = square.neighbour(Direction::SW);
        assert!(neighbour.is_none());
    }

    #[test]
    fn b2_has_correct_south_west_neighbour() {
        let square: Square = "b2".parse().unwrap();
        let neighbour = square.neighbour(Direction::SW);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Square::new(0, 0));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn a8_has_no_north_west_neighbour() {
        let square: Square = "A8".parse().unwrap();
        let neighbour = square.neighbour(Direction::NW);
        assert!(neighbour.is_none());
    }

    #[test]
    fn b7_has_correct_north_west_neighbour() {
        let square: Square = "B7".parse().unwrap();
        let neighbour = square.neighbour(Direction::NW);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Square::new(0, 7));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn h1_has_no_south_east_neighbour() {
        let square: Square = "h1".parse().unwrap();
        let neighbour = square.neighbour(Direction::SE);
        assert!(neighbour.is_none());
    }

    #[test]
    fn g2_has_correct_south_east_neighbour() {
        let square: Square = "G2".parse().unwrap();
        let neighbour = square.neighbour(Direction::SE);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Square::new(7, 0));
        } else {
            assert!(false);
        }
    }
}
