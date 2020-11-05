mod axis;
mod direction;

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

// Implementation of trait's methods on BitBoard
impl Default for BitBoard {
    fn default() -> Self {
        Self {
            b: [0, 0, 0]
        }
    }
}

impl Shl<u32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        self.shift_left(rhs as usize)
    }
}

impl Shl<i32> for BitBoard {
    type Output = Self;

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

    fn shr(self, rhs: u32) -> Self::Output {
        self.shift_right(rhs as usize)
    }
}

impl Shr<i32> for BitBoard {
    type Output = Self;

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

    #[allow(unused)]
    fn bitor(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    #[allow(unused)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    #[allow(unused)]
    fn bitand(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { b: [!self.b[0], !self.b[1], !self.b[2]] }
    }
}

// Homemade methods implemented on BitBoard
impl BitBoard {
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
    pub fn dilate(&self, dir: Direction) -> Self {
        unimplemented!()
    }

    // Methods for erosion
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
        match direction {
            // TODO: Finish this implementation.
            Direction::N => {}
            Direction::S => {}
            Direction::E => {}
            Direction::W => {}
            Direction::NE => {}
            Direction::NW => {}
            Direction::SE => {}
            Direction::SW => self.shift_left(by)
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