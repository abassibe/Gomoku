use super::axis::Axis;
use std::{fmt, slice::Iter};

const ARRAY_SIZE: usize = 8;

// TODO: Missing doc here
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
    All
}

impl Direction {
    pub fn to_axis(&self) -> Axis {
        match self {
            Direction::N => Axis::Vertical,
            Direction::S => Axis::Vertical,
            Direction::E => Axis::Horizontal,
            Direction::W => Axis::Horizontal,
            Direction::NE => Axis::DiagUpRight,
            Direction::SW => Axis::DiagUpRight,
            Direction::NW => Axis::DiagUpLeft,
            Direction::SE => Axis::DiagUpLeft,
            Direction::All => Axis::All
        }
    }

    pub fn to_invert(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
            Direction::NE => Direction::SW,
            Direction::SW => Direction::NE,
            Direction::NW => Direction::SE,
            Direction::SE => Direction::NW,
            Direction::All => Direction::All
        }
    }
}

impl From<Axis> for Direction {
    fn from(axis: Axis) -> Self {
        axis.to_direction()
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stringified = match self {
            Direction::N => "North",
            Direction::S => "South",
            Direction::E => "East",
            Direction::W => "West",
            Direction::NE => "North East",
            Direction::NW => "North West",
            Direction::SE => "South East",
            Direction::SW => "South West",
            Direction::All => "All directions"
        };
        write!(f, "{}", stringified)
    }
}

// TODO: Missing doc here
/// This struct exist only to build an iterator around Direction.
pub struct DirectionIterator {
    index_forward: isize,
    index_backward: isize,
    directions: [Direction; ARRAY_SIZE]
}

impl DirectionIterator {
    pub fn new() -> Self {
        Self {
            index_forward: 0,
            index_backward: ARRAY_SIZE as isize - 1,
            directions: [
                Direction::N,
                Direction::S,
                Direction::E,
                Direction::W,
                Direction::NE,
                Direction::NW,
                Direction::SE,
                Direction::SW
            ],
        }
    }

    pub fn as_array_iter() -> Iter<'static, Direction> {
        [
            Direction::N,
            Direction::S,
            Direction::E,
            Direction::W,
            Direction::NE,
            Direction::NW,
            Direction::SE,
            Direction::SW
        ]
        .iter()
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_forward > self.index_backward {
            return None;
        }

        let direction = self.directions[self.index_forward as usize];
        self.index_forward += 1;
        Some(direction)
    }
}

impl DoubleEndedIterator for DirectionIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index_backward < self.index_forward {
            return None;
        }

        let direction = self.directions[self.index_backward as usize];
        self.index_backward -= 1;
        Some(direction)
    }
}
