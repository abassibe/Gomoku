use super::axis::Axis;

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

// TODO: Missing doc here
/// This struct exist only to build an iterator around Direction.
pub struct DirectionIterator {
    index: usize,
    directions: [Direction; ARRAY_SIZE]
}

impl DirectionIterator {
    pub fn new() -> Self {
        Self {
            index: 0,
            directions: [Direction::N, Direction::S, Direction::E, Direction::W, Direction::NE, Direction::NW, Direction::SE, Direction::SW]
        }
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= ARRAY_SIZE {
            return None;
        }

        let direction = self.directions[self.index];
        self.index += 1;
        Some(direction)
    }
}