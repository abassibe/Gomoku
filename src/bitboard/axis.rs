use std::slice::Iter;

use super::direction::Direction;

const ARRAY_SIZE: usize = 4;

/// The enum which represent either one of the four possible axis.
/// Which repectively are:
/// - West <-> East
/// - North <-> South
/// - North West <-> South East
/// - South West <-> North East
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
    DiagUpLeft,
    DiagUpRight,
    All
}

impl Axis {
    pub fn to_direction(self) -> Direction {
        match self {
            Axis::Horizontal => Direction::W,
            Axis::Vertical => Direction::N,
            Axis::DiagUpLeft => Direction::NW,
            Axis::DiagUpRight => Direction::NE,
            Axis::All => Direction::All
        }
    }
}

impl From<Direction> for Axis {
    fn from(dir: Direction) -> Self {
        dir.to_axis()
    }
}

/// This is the struct which wraps around Axis to make it itarable.
/// When iterate over an AxisIterator, the values that pop out should repectively be:
///  - `Direction::W`
///  - `Direction::N`
///  - `Direction::NW`
///  - `Direction::NE`
pub struct AxisIterator {
    index: usize,
    axises: [Direction; ARRAY_SIZE]
}

impl AxisIterator {
    pub fn new() -> Self {
        Self {
            index: 0,
            axises: [Direction::W, Direction::N, Direction::NW, Direction::NE]
        }
    }

    pub fn as_array_iter() -> Iter<'static, Direction> {
        [Direction::W, Direction::N, Direction::NW, Direction::NE].iter()
    }
}

impl Iterator for AxisIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= ARRAY_SIZE {
            return None;
        }

        let direction = self.axises[self.index];
        self.index += 1;
        Some(direction)
    }
}