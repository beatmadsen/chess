use name::Name;

// TODO: state machine to keep track of relevant game state
pub trait Piece {
    fn legal_moves(&self) -> Vec<(Name, Name)>;

    fn box_clone(&self) -> Box<Piece>;
}

impl Clone for Box<Piece> {
    fn clone(&self) -> Box<Piece> {
        self.box_clone()
    }
}

pub struct Pawn {}
pub struct Queen {}
pub struct King {}
pub struct Knight {}
pub struct Rook {}

#[derive(Clone)]
pub struct Bishop {
    location: Name,
}

impl Piece for Bishop {
    fn legal_moves(&self) -> Vec<(Name, Name)> {
        use direction::Direction::*;
        vec![NE, NW, SW, SE]
            .iter()
            .flat_map(|d| self.location.collect_neighbours(d.clone()))
            .map(|neighbour| (self.location, neighbour))
            .collect()
    }

    fn box_clone(&self) -> Box<Piece> {
        Box::new((*self).clone())
    }
}
