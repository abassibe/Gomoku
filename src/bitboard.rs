pub(crate) mod axis;
pub(crate) mod direction;
pub(crate) mod pattern;

#[cfg(test)]
mod tests;

use std::{
    fmt,
    mem::size_of,
    ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub, BitOrAssign, BitXorAssign, BitAndAssign, Index}
};
use direction::*;
use axis::*;

const BITS_IN_U128: usize = size_of::<u128>() * 8;

// TODO: Implement method to get/set ~one or~ several bits by index
// TODO: Implement method to get/set one or several bits by coordonate (X, Y flatten to index then call previous method above)
// TODO: Implement mehtod to perform pattern matching!
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BitBoard {
    b: [u128; 3]
}

// ----------------------------------------
// Homemade methods implemented on BitBoard
// ----------------------------------------

// #region Methods
impl BitBoard {
    const NUMBER_OF_COLS: u32 = 19;
    /// This is the value used to perform a bitshift on a BitBoard
    /// when we have to move UP or DOWN all bit set on the BitBoard.
    /// This value is the number of columns in the BitBoard + 1
    /// (to take in account the separating bit at the end of each row).
    const MOVE_UP_DOWN_SHIFT_VALUE: u32 = Self::NUMBER_OF_COLS + 1;
    const INDEX_RETURN_FALSE: bool = false;
    const INDEX_RETURN_TRUE: bool = true;
    const ENDLINE_DELIMITER_MASK: Self = Self {
        b: [
            0b11111111111111111110111111111111111111101111111111111111111011111111111111111110111111111111111111101111111111111111111011111111,
            0b11111111111011111111111111111110111111111111111111101111111111111111111011111111111111111110111111111111111111101111111111111111,
            0b11101111111111111111111011111111111111111110111111111111111111101111111111111111111011111111111111111110111111111111111111100000
        ]
    };
    const FIRST_BIT_SET: Self = Self {
        b: [
            0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
            0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
        ]
    };
    const U128_FIRST_BIT_SET: u128 = 0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;

    // ------------
    // Constructors
    // ------------
    // #region Constructors
    pub fn new(one: u128, two: u128, three: u128) -> Self {
        Self { b: [one, two, three] }
    }

    pub fn from_array(from: [u128; 3]) -> Self {
        Self { b: from }
    }

    pub fn full() -> Self {
        !Self::default()
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_str(from: &str) -> Self {
        let mut one_line_str: String = from.split_ascii_whitespace()
            .fold(String::new(), |mut r, n| {
                r.push_str(n);
                if n.len() == 19 {
                    r.push('0');
                }
                r
            });
        let bits_count = one_line_str.len();
        for _ in 0..(BITS_IN_U128 * 3) - bits_count {
            one_line_str.push('0');
        }
        // let mut bins = vec![String::from("0b"), String::from("0b"), String::from("0b")];
        let mut bins: Vec<&str> = vec!();
        for i in 0..3 {
            bins.push(&one_line_str[BITS_IN_U128 * i..BITS_IN_U128 * (i + 1)]);
        }
        BitBoard::from_array([
            u128::from_str_radix(bins[0], 2).unwrap(),
            u128::from_str_radix(bins[1], 2).unwrap(),
            u128::from_str_radix(bins[2], 2).unwrap()
        ])
    }
    // #endregion Constructors

    pub fn enumerate(mut self) -> Vec<BitBoard> {
        let mut list: Vec<BitBoard> = Vec::with_capacity(40);
        let mut index = 0u32;

        while self.is_any() {
            if (&self & &Self::FIRST_BIT_SET).is_any() {
                list.push(Self::FIRST_BIT_SET >> index);
            }

            self = self << 1;
            index += 1;
        }

        list
    }

    // -------------
    // Tests methods
    // -------------
    // #region Test methods
    /// Returns `true` if **every** bits are set to 1 in the bitboard.
    /// Returns `false` otherwise.
    pub fn is_full(&self) -> bool {
        self & &BitBoard::ENDLINE_DELIMITER_MASK == BitBoard::ENDLINE_DELIMITER_MASK 
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

    /// Returns `true` if **at least** one bit is set to 1 in the bitboard.
    /// Returns `false` otherwise.
    pub fn is_any(&self) -> bool {
        !self.is_empty()
    }
    // #endregion Test methods

    // ---------------------------------
    // Computation's method for BitBoard
    // ---------------------------------
    pub fn compute_to_isize<F: Fn(&Self) -> isize>(&self, f: F) -> isize {
        f(self)
    }

    // ----------
    // Count bits
    // ----------
    pub fn count_ones(&self) -> u16 {
        self.b.iter().fold(0, |acc: u16, &x| acc + x.count_ones() as u16)
    }

    // ----------
    // Bit setter
    // ----------
    pub fn set(&mut self, bit_index: isize, bit_value: bool) {
        let max_index = (BITS_IN_U128 * self.b.len()) as isize;
        let min_index = -max_index;
        if bit_index >= max_index || bit_index < min_index {
            return;
        }
        let requested_bit = match bit_index.is_negative() {
            true => BitBoard::from_array([0, 0, 1]) << (bit_index.abs() - 1) as usize,
            false => BitBoard::from_array([1 << 127, 0, 0]) >> bit_index as usize
        };
        let is_bit_set = (*self & requested_bit).is_any();
        let mut new_self = self;
        if (is_bit_set && !bit_value) || (!is_bit_set && bit_value) {
            new_self ^= &requested_bit;
        }
    }

    // ----------
    // Bit getter
    // ----------
    /// Returns the index of every bit set in the BitBoard in a Vec<u16>.
    pub fn get_bit_indexes(&self) -> Vec<u16> {
        let mut result = vec![];
        let mut index = 0u16;
        let mut bitboard = *self;

        while bitboard.is_any() {
            if bitboard.b[0] & Self::U128_FIRST_BIT_SET != 0 {
                result.push(index);
            }
            bitboard = bitboard << 1;
            index += 1;
        }

        result
    }


    // ---------------------------------------
    // Implementation of bitshift for BitBoard
    // ---------------------------------------
    // #region Bitshift implem
    #[inline]
    fn shift_left(&self, by: usize) -> Self {
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
    fn shift_right(&self, by: usize) -> Self {
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

    // TODO: Missing doc here
    fn shift_direction(&self, direction: Direction) -> Self {
        let board = *self;
        *match direction {
            Direction::N => board << Self::MOVE_UP_DOWN_SHIFT_VALUE,
            Direction::S => board >> Self::MOVE_UP_DOWN_SHIFT_VALUE,
            Direction::E => board >> 1,
            Direction::W => board << 1,
            Direction::NE => board << Self::MOVE_UP_DOWN_SHIFT_VALUE - 1,
            Direction::NW => board << Self::MOVE_UP_DOWN_SHIFT_VALUE + 1,
            Direction::SE => board >> Self::MOVE_UP_DOWN_SHIFT_VALUE + 1,
            Direction::SW => board >> Self::MOVE_UP_DOWN_SHIFT_VALUE - 1,
            Direction::All => unimplemented!("You MUST not use Direction::All with this method")
        }.apply_endline_delimiter_mask()
    }

    /// This method cleans all the bits set in the 20eme column
    /// which are the separating bits.
    fn apply_endline_delimiter_mask(&mut self) -> &Self {
        let mut new_self = self;
        new_self &= &Self::ENDLINE_DELIMITER_MASK;
        new_self
    }
    // #endregion Bitshift implem

    // --------------------
    // Methods for dilation
    // --------------------
    /// This method should remain private.
    /// Use the operator `+` instead.
    #[inline]
    fn dilate_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::All => {
                let mut result = *self;
                for d in DirectionIterator::new() {
                    result |= self << d;
                }
                result
            },
            d => *self | (self << d)
        }
    }

    /// This method should remain private.
    /// Use the operator `+` instead.
    #[inline]
    fn dilate_axis(&self, dir: Axis) -> Self {
        match dir {
            Axis::All => {
                let mut result = *self;
                for d in AxisIterator::new() {
                    result |= self << d;
                }
                result
            },
            d => *self | (self << d.to_direction())
        }
    }

    // -------------------
    // Methods for erosion
    // -------------------
    /// This method should remain private.
    /// Use the operator `-` instead.
    #[inline]
    fn erode_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::All => {
                let mut result = *self;
                for d in DirectionIterator::new() {
                    result &= self << d;
                }
                result
            },
            d => *self & (self << d)
        }
    }

    /// This method should remain private.
    /// Use the operator `-` instead.
    #[inline]
    fn erode_axis(&self, dir: Axis) -> Self {
        match dir {
            Axis::All => {
                let mut result = *self;
                for d in AxisIterator::new() {
                    println!("Result:\n{}", result);
                    result &= self << d;
                }
                result
            },
            d => *self & (self << d.to_direction())
        }
    }
}
// #endregion Method

// ----------------------------------------------
// Implementation of trait's methods on BitBoard.
// ----------------------------------------------

// #region Traits
impl Default for BitBoard {
    /// Create a new instance of an empty `BitBoard`
    fn default() -> Self {
        Self {
            b: [0, 0, 0]
        }
    }
}

// #region Trait bitshift

// #region Bitshift on the left
impl Shl<u32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the left on a `BitBoard` using a u32.
    fn shl(self, rhs: u32) -> Self::Output {
        self.shift_left(rhs as usize)
    }
}

impl Shl<u32> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the left on a `BitBoard`'s reference using a u32.
    fn shl(self, rhs: u32) -> Self::Output {
        self.shift_left(rhs as usize)
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the left on a `BitBoard` using a usize.
    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl Shl<usize> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the left on a `BitBoard`'s reference using a usize.
    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl Shl<i32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the left on a `BitBoard` using a i32.
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

    /// Perform bitshift operation to the left on a `BitBoard`'s reference using a i32.
    fn shl(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            self.shift_right(rhs.abs() as usize)
        } else {
            self.shift_left(rhs as usize)
        }
    }
}

impl Shl<Direction> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the left on a `BitBoard` using a `Direction`.
    fn shl(self, rhs: Direction) -> Self::Output {
        self.shift_direction(rhs)
    }
}

impl Shl<Direction> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the left on a `BitBoard`'s reference using a `Direction`.
    fn shl(self, rhs: Direction) -> Self::Output {
        self.shift_direction(rhs)
    }
}
// #endregion Bitshift on the left

// #region Bitshift on the right
impl Shr<u32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the right on a `BitBoard` using a u32.
    fn shr(self, rhs: u32) -> Self::Output {
        self.shift_right(rhs as usize)
    }
}

impl Shr<u32> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the right on a `BitBoard`'s reference using a u32.
    fn shr(self, rhs: u32) -> Self::Output {
        self.shift_right(rhs as usize)
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the right on a `BitBoard` using a usize.
    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl Shr<usize> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the right on a `BitBoard`'s reference using a usize.
    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl Shr<i32> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the right on a `BitBoard` using a i32.
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

    /// Perform bitshift operation to the right on a `BitBoard`'s reference using a i32.
    fn shr(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            self.shift_left(rhs.abs() as usize)
        } else {
            self.shift_right(rhs as usize)
        }
    }
}

impl Shr<Direction> for BitBoard {
    type Output = Self;

    /// Perform bitshift operation to the right on a `BitBoard` using a `Direction`.
    fn shr(self, rhs: Direction) -> Self::Output {
        self.shift_direction(rhs)
    }
}

impl Shr<Direction> for &BitBoard {
    type Output = BitBoard;

    /// Perform bitshift operation to the right on a `BitBoard`'s reference using a `Direction`.
    fn shr(self, rhs: Direction) -> Self::Output {
        self.shift_direction(rhs)
    }
}
// #endregion Bitshift on the right
// #endregion Trait bitshift

// #region Trait bitwise op
impl BitOr for BitBoard {
    type Output = Self;

    /// Perform bitwise operation OR between two `BitBoards`.
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] | rhs.b[0], self.b[1] | rhs.b[1], self.b[2] | rhs.b[2]] }
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation OR between two `BitBoard`'s references.
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] | rhs.b[0], self.b[1] | rhs.b[1], self.b[2] | rhs.b[2]] }
    }
}

impl BitOrAssign for BitBoard {
    /// Perform an in place bitwise operation OR between two `BitBoard`s.
    fn bitor_assign(&mut self, rhs: Self) {
        self.b[0] |= rhs.b[0];
        self.b[1] |= rhs.b[1];
        self.b[2] |= rhs.b[2];
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    /// Perform an in place bitwise operation OR between two `BitBoard`'s references.
    fn bitor_assign(&mut self, rhs: &Self) {
        self.b[0] |= rhs.b[0];
        self.b[1] |= rhs.b[1];
        self.b[2] |= rhs.b[2];
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    /// Perform bitwise operation XOR between two `BitBoard`s.
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] ^ rhs.b[0], self.b[1] ^ rhs.b[1], self.b[2] ^ rhs.b[2]] }
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation XOR between two `BitBoard`'s references.
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] ^ rhs.b[0], self.b[1] ^ rhs.b[1], self.b[2] ^ rhs.b[2]] }
    }
}

impl BitXorAssign for BitBoard {
    /// Perform an in place bitwise operation XOR between two `BitBoard`s.
    fn bitxor_assign(&mut self, rhs: Self) {
        self.b[0] ^= rhs.b[0];
        self.b[1] ^= rhs.b[1];
        self.b[2] ^= rhs.b[2];
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    /// Perform an in place bitwise operation XOR between two `BitBoard`'s references.
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.b[0] ^= rhs.b[0];
        self.b[1] ^= rhs.b[1];
        self.b[2] ^= rhs.b[2];
    }
}

impl BitXorAssign<&BitBoard> for &mut BitBoard {
    /// Perform an in place bitwise operation XOR between two `BitBoard`'s references.
    fn bitxor_assign(&mut self, rhs: &BitBoard) {
        self.b[0] ^= rhs.b[0];
        self.b[1] ^= rhs.b[1];
        self.b[2] ^= rhs.b[2];
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    /// Perform bitwise operation AND between two `BitBoards`.
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] & rhs.b[0], self.b[1] & rhs.b[1], self.b[2] & rhs.b[2]] }
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation AND between two `BitBoard`'s references.
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output { b: [self.b[0] & rhs.b[0], self.b[1] & rhs.b[1], self.b[2] & rhs.b[2]] }
    }
}

impl BitAndAssign for BitBoard {
    /// Perform an in place bitwise operation OR between two `BitBoard`s.
    fn bitand_assign(&mut self, rhs: Self) {
        self.b[0] &= rhs.b[0];
        self.b[1] &= rhs.b[1];
        self.b[2] &= rhs.b[2];
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    /// Perform an in place bitwise operation OR between two `BitBoard`'s references.
    fn bitand_assign(&mut self, rhs: &Self) {
        self.b[0] &= rhs.b[0];
        self.b[1] &= rhs.b[1];
        self.b[2] &= rhs.b[2];
    }
}

impl BitAndAssign<&BitBoard> for &mut BitBoard {
    /// Perform an in place bitwise operation OR between two `BitBoard`'s references.
    fn bitand_assign(&mut self, rhs: &BitBoard) {
        self.b[0] &= rhs.b[0];
        self.b[1] &= rhs.b[1];
        self.b[2] &= rhs.b[2];
    }
}

impl Not for BitBoard {
    type Output = Self;

    /// Perform bitwise operation NOT on a `BitBoard`.
    fn not(self) -> Self::Output {
        *Self::Output { b: [!self.b[0], !self.b[1], !self.b[2]] }.apply_endline_delimiter_mask()
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    /// Perform bitwise operation NOT on a `BitBoard`'s reference.
    fn not(self) -> Self::Output {
        *BitBoard { b: [!self.b[0], !self.b[1], !self.b[2]] }.apply_endline_delimiter_mask()
    }
}
// #endregion Trait bitwise op

// #region Trait add for dilation
impl Add<Direction> for BitBoard {
    type Output = Self;

    /// Perform a dilation on a `BitBoard` using the provided `Direction`
    fn add(self, rhs: Direction) -> Self::Output {
        self.dilate_dir(rhs)
    }
}

impl Add<Direction> for &BitBoard {
    type Output = BitBoard;

    /// Perform a dilation on a `BitBoard`'s reference using the provided `Direction`
    fn add(self, rhs: Direction) -> Self::Output {
        self.dilate_dir(rhs)
    }
}

impl Add<Axis> for BitBoard {
    type Output = Self;

    /// Perform a dilation on a `BitBoard` using the provided `Direction`
    fn add(self, rhs: Axis) -> Self::Output {
        self.dilate_axis(rhs)
    }
}

impl Add<Axis> for &BitBoard {
    type Output = BitBoard;

    /// Perform a dilation on a `BitBoard`'s reference using the provided `Direction`
    fn add(self, rhs: Axis) -> Self::Output {
        self.dilate_axis(rhs)
    }
}
// #endregion Trait add for dilation

// #region Trait sub for erosion
impl Sub<Direction> for BitBoard {
    type Output = Self;

    /// Perform an erosion on a `BitBoard` using the provided `Direction`
    fn sub(self, rhs: Direction) -> Self::Output {
        self.erode_dir(rhs)
    }
}

impl Sub<Direction> for &BitBoard {
    type Output = BitBoard;

    /// Perform an erosion on a `BitBoard`'s reference using the provided `Direction`
    fn sub(self, rhs: Direction) -> Self::Output {
        self.erode_dir(rhs)
    }
}

impl Sub<Axis> for BitBoard {
    type Output = Self;

    /// Perform an erosion on a `BitBoard` using the provided `Direction`
    fn sub(self, rhs: Axis) -> Self::Output {
        self.erode_axis(rhs)
    }
}

impl Sub<Axis> for &BitBoard {
    type Output = BitBoard;

    /// Perform an erosion on a `BitBoard`'s reference using the provided `Direction`
    fn sub(self, rhs: Axis) -> Self::Output {
        self.erode_axis(rhs)
    }
}
// #endregion Trait sub for erosion

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str_repr = format!("{:0128b}{:0128b}{:0128b}", self.b[0], self.b[1], self.b[2]);
        let mut str_vec: Vec<String> = vec![];
        let mut result = Ok(());

        for _ in 0..19 {
            str_vec.push(str_repr[..19].into());
            str_repr = str_repr[20..].into();
        }

        for s in str_vec.iter() {
            result = Ok(write!(f, "{}\n", s)?);
        }

        result
    }
}

// #region Trait index
impl Index<usize> for BitBoard {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= BITS_IN_U128 * self.b.len() {
            return &Self::INDEX_RETURN_FALSE;
        }
        let requested_bit = BitBoard::from_array([1 << 127, 0, 0]) >> index;
        if (self & &requested_bit).is_any() {
            &Self::INDEX_RETURN_TRUE
        }
        else {
            &Self::INDEX_RETURN_FALSE
        }
    }
}

impl Index<isize> for BitBoard {
    type Output = bool;

    fn index(&self, index: isize) -> &Self::Output {
        let max_index = (BITS_IN_U128 * self.b.len()) as isize;
        let min_index = -max_index;
        if index >= max_index || index < min_index {
            return &Self::INDEX_RETURN_FALSE;
        }
        let requested_bit = match index.is_negative() {
            true => BitBoard::from_array([0, 0, 1]) << (index.abs() - 1) as usize,
            false => BitBoard::from_array([1 << 127, 0, 0]) >> index as usize
        };
        if (self & &requested_bit).is_any() {
            &Self::INDEX_RETURN_TRUE
        }
        else {
            &Self::INDEX_RETURN_FALSE
        }
    }
}

impl Index<u32> for BitBoard {
    type Output = bool;

    fn index(&self, index: u32) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<i32> for BitBoard {
    type Output = bool;

    fn index(&self, index: i32) -> &Self::Output {
        &self[index as isize]
    }
}

impl Index<(usize, usize)> for BitBoard {
    type Output = bool;

    // Here we assume that (0, 0) is at the top left corner of the board
    // with x for the horizontal axis
    // and y for the vertical axis
    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        let x = coord.0;
        let y = coord.1;
        if x >= Self::NUMBER_OF_COLS as usize || y >= Self::NUMBER_OF_COLS as usize {
            return &Self::INDEX_RETURN_FALSE;
        }
        &self[y * Self::MOVE_UP_DOWN_SHIFT_VALUE as usize + x]
    }
}

impl Index<(u32, u32)> for BitBoard {
    type Output = bool;

    // Here we assume that (0, 0) is at the top left corner of the board
    // with x for the horizontal axis
    // and y for the vertical axis
    fn index(&self, coord: (u32, u32)) -> &Self::Output {
        &self[(coord.0 as usize, coord.1 as usize)]
    }
}
// #endregion Trait index

// #endregion Traits