use super::{BitBoard, direction::Direction};

pub struct Pattern {
    at_start_line: BitBoard,
    current: BitBoard,
    // A BitBoard that contains the bits which MUST not be set
    // in order to actually match the pattern.
    // Must have the same origin as the pattern itself.
    must_be_empty: Option<BitBoard>,
    bits_in_pattern: u16,
    current_x: u8,
    current_y: u8,
    found: bool
}

impl Pattern {
    pub fn from_str(from: &str, empty: Option<&str>) -> Self {
        let pattern = BitBoard::from_str(from);
        let must_be_empty = empty.and_then(|s| Some(BitBoard::from_str(s)));

        Self::new(pattern, must_be_empty)
    }

    pub fn new(pattern: BitBoard, must_be_empty: Option<BitBoard>) -> Self {
        Pattern {
            at_start_line: pattern,
            current: pattern,
            must_be_empty,
            bits_in_pattern: pattern.count_ones(),
            current_x: 0,
            current_y: 0,
            found: false
        }
    }

    /// **IMPORTANT**: This method modifies the underlying BitBoard
    /// during the pattern search, thus the pattern should **NOT** be reused.
    pub fn search_in(&mut self, board: &BitBoard) -> bool {
        loop {
            if board & &self.current == self.current {
                if self.must_be_empty.is_none() {
                    self.found = true;
                    return true;
                }
                if let Some(empty_pattern) = self.must_be_empty {
                    if (board & &((empty_pattern >> (BitBoard::MOVE_UP_DOWN_SHIFT_VALUE * self.current_y as u32)) >> self.current_x as u32)).is_empty() {
                        self.found = true;
                        return true;
                    }
                }
            }
            if !self.try_move_by_one() {
                return false;
            }
        }
    }

    pub fn get_pattern_coord(&self) -> (u8, u8) {
        (self.current_x, self.current_y)
    }

    pub fn is_match(&self) -> bool {
        self.found
    }

    /// Try to move the pattern in the BitBoard by one bit.
    /// If this move would make the pattern to goes out of the board,
    /// then the pattern is moved back to the left-most position and moved
    /// down by one line.
    /// If the pattern can't move any more, this method returns `false`.
    /// It returns `true` in any other cases.
    fn try_move_by_one(&mut self) -> bool {
        let moved = self.current >> Direction::E;

        if moved.count_ones() == self.bits_in_pattern {
            self.current = moved;
            self.current_x += 1;
            return true;
        }

        let moved_down = self.at_start_line >> Direction::S;
        if moved_down.count_ones() == self.bits_in_pattern {
            self.at_start_line = moved_down;
            self.current = moved_down;
            self.current_x = 0;
            self.current_y += 1;
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::{Pattern, BitBoard};

    #[test]
    fn test_method_match_in_match_full_square() {
        // Arrange
        // This is a simple pattern, just a 2 by 2 square.
        let mut pattern = Pattern::from_str("
            1100000000000000000
            1100000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", None);
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000011
            0000000000000000011
        ");
        let expected = true;
        let expected_pos = (17u8, 17u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }

    #[test]
    fn test_method_match_in_match_full_cross() {
        // Arrange
        let mut pattern = Pattern::from_str("
            0010000000000000000
            0010000000000000000
            1111100000000000000
            0010000000000000000
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", None);
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000001000000
            0000000000001000000
            0000000000111110000
            0000000000001000000
            0000000000001000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = true;
        let expected_pos = (10u8, 12u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }

    #[test]
    fn test_method_match_in_doesnt_match_partial_pattern() {
        // Arrange
        let mut pattern = Pattern::from_str("
            0010000000000000000
            0010000000000000000
            1111100000000000000
            0010000000000000000
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", None);
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000010
            0000000000000000010
            0000000000000001111
            0000000000000000010
            0000000000000000010
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = false;
        // This is the max pos the pattern can goes at
        let expected_pos = (14u8, 14u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }

    #[test]
    fn test_method_match_in_match_full_cross_in_garbage() {
        // Arrange
        let mut pattern = Pattern::from_str("
            0010000000000000000
            0010000000000000000
            1111100000000000000
            0010000000000000000
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", None);
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            1111100000000000000
            1111100000000000000
            1111100000000000000
            1111100000000000000
            1111100000000000000
        ");
        let expected = true;
        let expected_pos = (0u8, 14u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }

    #[test]
    fn test_method_match_in_match_pattern_with_two_part() {
        // Arrange
        let mut pattern = Pattern::from_str("
            1000000000000000000
            0100000000000000000
            0000000000000000000
            0001000000000000000
            0000100000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", None);
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000010000
            0000000000000001000
            0000000000000000000
            0000000000000000010
            0000000000000000001
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = true;
        let expected_pos = (14u8, 5u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }

    #[test]
    fn test_method_match_in_match_pattern_with_two_part_and_must_empty_bits() {
        // Arrange
        let mut pattern = Pattern::from_str("
            1000000000000000000
            0100000000000000000
            0000000000000000000
            0001000000000000000
            0000100000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", Some("
            0000000000000000000
            0000000000000000000
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        "));
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000010000
            0000000000000001000
            0000000000000000000
            0000000000000000010
            0000000000000000001
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = true;
        let expected_pos = (14u8, 5u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }

    #[test]
    fn test_method_match_in_match_pattern_with_two_part_and_must_empty_bits_in_garbage() {
        // Arrange
        let mut pattern = Pattern::from_str("
            1000000000000000000
            0100000000000000000
            0000000000000000000
            0001000000000000000
            0000100000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ", Some("
            0000000000000000000
            0000000000000000000
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        "));
        let bitboard = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000011111
            0000000000000011111
            0000000000000011111
            0000000000000011111
            0000000000000011111
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = false;
        let expected_pos = (14u8, 14u8);

        // Act
        let result = pattern.search_in(&bitboard);
        let pos = pattern.get_pattern_coord();
        let found = pattern.is_match();

        // Assert
        assert_eq!(expected, result);
        assert_eq!(expected_pos, pos);
        assert_eq!(expected, found);
    }
}