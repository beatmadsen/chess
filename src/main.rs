fn main() {
    let i: i32 = 0i32 - 1u8 as i32 - 4u16 as i32;
    println!("Hello, world! {}", i);
}

/*
internal representation of column is integer,
can be transformed to char for display purposes
*/
#[derive(PartialEq, Debug)]
pub struct Square {
    column: u8,
    row: u8,
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
    pub fn new(column: u8, row: u8) -> Square {
        Square {
            column: column,
            row: row,
        }
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
        let (column, row) = (self.column as i8 + d_column, self.row as i8 + d_row);
        match (column, row) {
            (0...7, 0...7) => Some(Square {
                row: row as u8,
                column: column as u8,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g7_has_correct_north_east_neighbour() {
        let square = Square::new(6, 6);
        let neighbour = square.neighbour(Direction::NE);
        if let Some(neighbour) = neighbour {
            assert_eq!(neighbour, Square::new(7, 7));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn h8_has_correct_north_east_neighbour() {
        let square = Square::new(7, 7);
        let neighbour = square.neighbour(Direction::NE);
        assert!(neighbour.is_none());
    }
}
