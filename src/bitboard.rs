pub(crate) mod axis;
pub(crate) mod direction;

#[cfg(test)]
mod tests;

use std::{
    mem::size_of,
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr}
};
use direction::*;
use axis::*;

const BITS_IN_U128: usize = size_of::<u128>() * 8;

// TODO: Implement trait {Or,Xor,And}Assign
// TODO: Implement trait std::fmt::Display
// TODO: Implement trait Index
// TODO: Implement trait Eq/PartialEq
// TODO: Implement trait Shl/Shr<Direction>
// TODO: Implement method to get/set one or several bits by index
// TODO: Implement method to get/set one or several bits by coordonate (X, Y flatten to index then call previous method above)
#[derive(Debug, Copy, Clone)]
pub struct BitBoard {
    b: [u128; 3]
}

// ----------------------------------------------
// Implementation of trait's methods on BitBoard.
// ----------------------------------------------
impl Default for BitBoard {
    /// Create a new instance of an empty BitBoard
    fn default() -> Self {
        Self {
            b: [0, 0, 0]
        }
    }
}

impl Shl<u32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the left on a BitBoard using a u32.
    fn shl(self, rhs: u32) -> Self::Output {
        self.shift_left(rhs as usize)
    }
}

impl Shl<u32> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the left on a BitBoard's reference using a u32.
    fn shl(self, rhs: u32) -> Self::Output {
        self.shift_left(rhs as usize)
    }
}

impl Shl<i32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the left on a BitBoard using a i32.
    fn shl(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            self.shift_right(rhs.abs() as usize)
        } else {
            self.shift_left(rhs as usize)
        }
    }
}

impl Shl<i32> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the left on a BitBoard's reference using a i32.
    fn shl(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            self.shift_right(rhs.abs() as usize)
        } else {
            self.shift_left(rhs as usize)
        }
    }
}

impl Shr<u32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the right on a BitBoard using a u32.
    fn shr(self, rhs: u32) -> Self::Output {
        self.shift_right(rhs as usize)
    }
}

impl Shr<u32> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the right on a BitBoard's reference using a u32.
    fn shr(self, rhs: u32) -> Self::Output {
        self.shift_right(rhs as usize)
    }
}

impl Shr<i32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the right on a BitBoard using a i32.
    fn shr(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            self.shift_left(rhs.abs() as usize)
        } else {
            self.shift_right(rhs as usize)
        }
    }
}

impl Shr<i32> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the right on a BitBoard's reference using a i32.
    fn shr(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            self.shift_left(rhs.abs() as usize)
        } else {
            self.shift_right(rhs as usize)
        }
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    /// Perform bitwise operation OR between two BitBoards.
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] | rhs.b[0], self.b[1] | rhs.b[1], self.b[2] | rhs.b[2]] }
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation OR between two BitBoard's references.
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] | rhs.b[0], self.b[1] | rhs.b[1], self.b[2] | rhs.b[2]] }
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    /// Perform bitwise operation XOR between two BitBoards.
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] ^ rhs.b[0], self.b[1] ^ rhs.b[1], self.b[2] ^ rhs.b[2]] }
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation XOR between two BitBoard's references.
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] ^ rhs.b[0], self.b[1] ^ rhs.b[1], self.b[2] ^ rhs.b[2]] }
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    /// Perform bitwise operation AND between two BitBoards.
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] & rhs.b[0], self.b[1] & rhs.b[1], self.b[2] & rhs.b[2]] }
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation AND between two BitBoard's references.
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] & rhs.b[0], self.b[1] & rhs.b[1], self.b[2] & rhs.b[2]] }
    }
}

impl Not for BitBoard {
    type Output = Self;

    /// Perform bitwise operation NOT on a BitBoard.
    fn not(self) -> Self::Output {
        Self::Output { b: [!self.b[0], !self.b[1], !self.b[2]] }
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation NOT on a BitBoard's reference.
    fn not(self) -> Self::Output {
        Self::Output { b: [!self.b[0], !self.b[1], !self.b[2]] }
    }
}


// ----------------------------------------
// Homemade methods implemented on BitBoard
// ----------------------------------------
impl BitBoard {
    const MOVE_UP_DOWN_SHIFT_VALUE: u32 = 19;

    // Constructors
    pub fn new(one: u128, two: u128, three: u128) -> Self {
        Self { b: [one, two, three] }
    }

    pub fn full() -> Self {
        !Self::default()
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_array(from: [u128; 3]) -> Self { 
        Self { b: from }
    }

    // Computation's methods for BitBoard
    pub fn compute_to_isize<F: Fn(&Self) -> isize>(&self, f: F) -> isize {
        f(self)
    }

    // Implementation of bitshift for BitBoard
    #[inline]
    pub fn shift_left(&self, by: usize) -> Self {
        let bits = self.b;
        let max_index = bits.len() - 1;
        let mut new_bits: [u128; 3] = [0, 0, 0];

        if by >= BITS_IN_U128 * (max_index + 1) {
            return Self::default();
        }

        let inner_lshift = by % BITS_IN_U128;
        let inner_rshift = BITS_IN_U128 - inner_lshift;
        let value_off = by / BITS_IN_U128;
        for (dest_i, src_i) in (0..=(max_index - value_off)).rev().zip((0..=max_index).rev()) {
            if src_i < max_index && inner_rshift < BITS_IN_U128 {
                new_bits[dest_i] = bits[src_i + 1] >> inner_rshift
            }
            new_bits[dest_i] |= bits[src_i] << inner_lshift;
        }

        Self {
            b: new_bits
        }
    }

    #[inline]
    pub fn shift_right(&self, by: usize) -> Self {
        let bits = self.b;
        let max_index = bits.len() - 1;
        let mut new_bits: [u128; 3] = [0, 0, 0];

        if by >= BITS_IN_U128 * (max_index + 1) {
            return Self::default();
        }

        let inner_rshift = by % BITS_IN_U128;
        let inner_lshift = BITS_IN_U128 - inner_rshift;
        let value_off = by / BITS_IN_U128;
        for (dest_i, src_i) in (value_off..=max_index).zip((0..=max_index)) {
            if src_i > usize::MIN && inner_lshift < BITS_IN_U128 {
                new_bits[dest_i] = bits[src_i - 1] << inner_lshift
            }
            new_bits[dest_i] |= bits[src_i] >> inner_rshift;
        }

        Self {
            b: new_bits
        }
    }

    // Methods for dilation
    #[inline]
    pub fn dilate(&self, dir: Direction) -> Self {
        *self | (self.shift_direction(dir))
    }

    // Methods for erosion
    #[inline]
    pub fn erode(&self, dir: Direction) -> Self {
        unimplemented!()
    }

    /// Returns `true` if **no** bits are set to 1 in the bitboard.
    /// Returns `false` otherwise.
    pub fn is_empty(&self) -> bool {
        for x in &self.b {
            if *x != 0 {
                return false;
            }
        }

        true
    }

    /// Returns `true` if **every** bits are set to 1 in the bitboard.
    /// Returns `false` otherwise.
    pub fn is_full(&self) -> bool {
        for x in &self.b {
            if *x != u128::MAX {
                return false
            }
        }

        true
    }

    // TODO: Missing doc here
    pub fn shift_direction(&self, direction: Direction) -> Self {
        let board = *self;
        match direction {
            // TODO: Finish this implementation.
            Direction::N => board << Self::MOVE_UP_DOWN_SHIFT_VALUE,
            Direction::S => board >> Self::MOVE_UP_DOWN_SHIFT_VALUE,
            Direction::E => board >> 1,
            Direction::W => board << 1,
            Direction::NE => board << Self::MOVE_UP_DOWN_SHIFT_VALUE - 1,
            Direction::NW => board << Self::MOVE_UP_DOWN_SHIFT_VALUE + 1,
            Direction::SE => board >> Self::MOVE_UP_DOWN_SHIFT_VALUE + 1,
            Direction::SW => board >> Self::MOVE_UP_DOWN_SHIFT_VALUE - 1,
            Direction::All => {
                let mut result = Self::default();

                for d in DirectionIterator::new() {
                    // TODO: Replace this `= result` with a `|=` when OrAssign will be implemented
                    result = result | self.shift_direction(d);
                }

                result
            }
        }
    }
}