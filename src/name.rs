use direction::Direction;
use error::ChessError;
use std::fmt;
use std::str::FromStr;

/*
internal representation of column is integer,
can be transformed to char for display purposes
*/
#[derive(PartialEq, Debug, Clone)]
pub struct Name {
    column: i8,
    row: i8,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, {}", self.column, self.row)
    }
}

fn deltas(d: Direction) -> (i8, i8) {
    match d {
        Direction::N => (0, 1),
        Direction::NE => (1, 1),
        Direction::E => (1, 0),
        Direction::SE => (1, -1),
        Direction::S => (0, -1),
        Direction::SW => (-1, -1),
        Direction::W => (-1, 0),
        Direction::NW => (-1, 1),
    }
}

impl Name {
    pub fn new(column: i8, row: i8) -> Name {
        match (column, row) {
            (0...7, 0...7) => Name {
                column: column,
                row: row,
            },
            _ => panic!("Row and column must have values between 0 and 7"),
        }
    }

    pub fn from_tuple(tuple: (i8, i8)) -> Name {
        Self::new(tuple.0, tuple.1)
    }

    pub fn neighbour(&self, d: Direction) -> Option<Name> {
        let (d_column, d_row) = deltas(d);
        let (column, row) = (self.column + d_column, self.row + d_row);
        match (column, row) {
            (0...7, 0...7) => Some(Name {
                row: row,
                column: column,
            }),
            _ => None,
        }
    }
}

impl FromStr for Name {
    type Err = ChessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 2 {
            return Err(ChessError::new("x"));
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
            .map(Name::from_tuple)
            .ok_or_else(|| ChessError::new("y"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g7_has_correct_north_east_neighbour() {
        let name: Name = "G7".parse().unwrap();
        let neighbour = name.neighbour(Direction::NE);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Name::new(7, 7));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn h8_has_no_north_east_neighbour() {
        let name = Name::new(7, 7);
        let neighbour = name.neighbour(Direction::NE);
        assert!(neighbour.is_none());
    }

    #[test]
    fn a1_has_no_south_west_neighbour() {
        let name = Name::new(0, 0);
        let neighbour = name.neighbour(Direction::SW);
        assert!(neighbour.is_none());
    }

    #[test]
    fn b2_has_correct_south_west_neighbour() {
        let name: Name = "b2".parse().unwrap();
        let neighbour = name.neighbour(Direction::SW);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Name::new(0, 0));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn a8_has_no_north_west_neighbour() {
        let name: Name = "A8".parse().unwrap();
        let neighbour = name.neighbour(Direction::NW);
        assert!(neighbour.is_none());
    }

    #[test]
    fn b7_has_correct_north_west_neighbour() {
        let name: Name = "B7".parse().unwrap();
        let neighbour = name.neighbour(Direction::NW);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Name::new(0, 7));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn h1_has_no_south_east_neighbour() {
        let name: Name = "h1".parse().unwrap();
        let neighbour = name.neighbour(Direction::SE);
        assert!(neighbour.is_none());
    }

    #[test]
    fn g2_has_correct_south_east_neighbour() {
        let name: Name = "G2".parse().unwrap();
        let neighbour = name.neighbour(Direction::SE);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Name::new(7, 0));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn d4_has_correct_north_neighbour() {
        let name: Name = "D4".parse().unwrap();
        let neighbour = name.neighbour(Direction::N);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, "D5".parse().unwrap());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn d4_has_correct_east_neighbour() {
        let name: Name = "D4".parse().unwrap();
        let neighbour = name.neighbour(Direction::E);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, "E4".parse().unwrap());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn d4_has_correct_south_neighbour() {
        let name: Name = "D4".parse().unwrap();
        let neighbour = name.neighbour(Direction::S);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, "d3".parse().unwrap());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn d4_has_correct_west_neighbour() {
        let name: Name = "D4".parse().unwrap();
        let neighbour = name.neighbour(Direction::W);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, "C4".parse().unwrap());
        } else {
            assert!(false);
        }
    }
}
