use crate::utils::geom::Direction::{Down, Left, Right, Up};

#[derive(Hash, Debug, PartialEq, Eq, Clone, Default)]
pub(crate) struct Coord {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) const DIRECTIONS: [Direction; 4] = [Left, Up, Right, Down];
