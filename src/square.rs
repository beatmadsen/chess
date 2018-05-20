use name::Name;

pub trait Piece {}

pub struct Pawn {}
pub struct Queen {}
pub struct King {}
pub struct Knight {}
pub struct Rook {}
pub struct Bishop {}

pub struct Square {
    piece: Option<Piece>,
    name: Name,
}

pub struct Position {
    squares: [[Square; 8]; 8],
}
