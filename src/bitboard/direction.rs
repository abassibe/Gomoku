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