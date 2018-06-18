use name::Name;
use piece::Piece;

pub struct Square {
    piece: Option<Box<Piece>>,
    name: Name,
}

impl Square {
    pub fn legal_moves(&self) -> Vec<(Name, Name)> {
        if let Some(piece) = &self.piece {
            piece.legal_moves()
        } else {
            vec![]
        }
    }
}

pub struct Position {
    squares: [[Square; 8]; 8],
}

impl Position {
    pub fn legal_moves(&self) -> Vec<(Name, Name)> {
        let mut moves = Vec::new();
        // not mapping, since we don't want to clone squares.
        for column in &self.squares {
            for square in column {
                moves.extend(square.legal_moves())
            }
        }
        moves
    }
}
