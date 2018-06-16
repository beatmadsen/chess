use name::Name;

pub trait Piece {}

pub struct Pawn {}
pub struct Queen {}
pub struct King {}
pub struct Knight {}
pub struct Rook {}
pub struct Bishop {}

pub struct Square {
    piece: Option<Box<Piece>>,
    name: Name,
}

impl Square {
    pub fn legal_moves(&self) -> Vec<(Name, Name)> {
        // TODO: implement this
        vec![("b2".parse().unwrap(), "b3".parse().unwrap())]
    }
}

pub struct Position {
    squares: [[Square; 8]; 8],
}

impl Position {
    pub fn legal_moves(&self) -> Vec<(Name, Name)> {
        let mut v: Vec<(Name, Name)> = Vec::new();
        // not mapping, since we don't want to clone squares.
        for column in &self.squares {
            for square in column {
                v.extend(square.legal_moves())
            }
        }
        v
    }
}
