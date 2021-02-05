use std::collections::{HashMap, hash_map::Iter};

use super::{*, direction::DirectionIterator};

const EDGE_MASK: BitBoard = BitBoard::new(
        340281880143881689085708262006044230272,
        207692072411988285641522779730903040,
        53169170537469001124229831611119566816
    );

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PatternName {
    CloseTwo,
    OpenThree,
    OpenSplitThreeLeft,
    OpenSplitThreeRight,
    OpenFour,
    CloseThree,
    CloseSplitThreeLeft,
    CloseSplitThreeRight,
    CloseFour,
    SplitFourLeft,
    SplitFourMiddle,
    SplitFourRight,
    Five
}

#[derive(Debug)]
pub struct NewPattern {
    patterns: HashMap<PatternName, (u8, u8, bool)>
}

impl NewPattern {
    pub fn new() -> NewPattern {
        let mut hashmap: HashMap<PatternName, (u8, u8, bool)> = HashMap::new();

        hashmap.insert(PatternName::CloseTwo,               (0b11000000, 3, false));
        hashmap.insert(PatternName::OpenThree,              (0b01110000, 5, true));
        hashmap.insert(PatternName::OpenSplitThreeRight,    (0b01101000, 6, false));
        hashmap.insert(PatternName::OpenSplitThreeLeft,     (0b01011000, 6, false));
        hashmap.insert(PatternName::OpenFour,               (0b01111000, 6, true));
        hashmap.insert(PatternName::CloseThree,             (0b11100000, 4, false));
        hashmap.insert(PatternName::CloseSplitThreeRight,   (0b11010000, 5, false));
        hashmap.insert(PatternName::CloseSplitThreeLeft,    (0b10110000, 5, false));
        hashmap.insert(PatternName::CloseFour,              (0b11110000, 5, false));
        hashmap.insert(PatternName::SplitFourLeft,          (0b01011100, 6, false));
        hashmap.insert(PatternName::SplitFourMiddle,        (0b01101100, 6, false));
        hashmap.insert(PatternName::SplitFourRight,         (0b01110100, 6, false));
        hashmap.insert(PatternName::Five,                   (0b11111000, 5, true));

        NewPattern {
            patterns: hashmap
        }
    }

    pub fn iter(&self) -> Iter<'_, PatternName, (u8, u8, bool)> {
        self.patterns.iter()
    }
}

impl Index<PatternName> for NewPattern {
    type Output = (u8, u8, bool);

    fn index(&self, name: PatternName) -> &Self::Output {
        &self.patterns[&name]
    }
}

// TODO: Implement the method to get the next move as soon as possible in order to test it and identify potential bugs
// TODO: Implement the missing methods (get the potential next moves according to the threats/opportunities, ...) in the right mod directly this time
// TODO: Test all this as much as possible
#[inline]
pub fn match_pattern_base(
    player: BitBoard,
    opponent: BitBoard,
    pattern: u8,
    pattern_size: u8,
    move_step: u8,
    closure_bits: u8,
    open_cells: BitBoard,
    direction: Direction
) -> BitBoard {
    let (mut result, mut edge_mask) = if closure_bits == U8_FIRST_BIT {
        (opponent, BitBoard::empty())
    } else {
        (BitBoard::full(), EDGE_MASK << direction << direction.to_invert())
    };
    let mut x = 0;

    while x < pattern_size && result.is_any() {
        result = if ((pattern << x) & U8_FIRST_BIT) == U8_FIRST_BIT {
            (result >> direction) & player
        } else {
            let inner_result = ((result >> direction) | edge_mask) & open_cells;
            edge_mask = BitBoard::empty();
            inner_result
        };
        x += 1;
    }

    result.shift_direction_by(direction.to_invert(), move_step)// & open_cells
}

pub fn match_pattern_all_directions(
    player: BitBoard,
    opponent: BitBoard,
    pattern: u8,
    pattern_size: u8,
    move_step: u8,
    closure_bits: u8
) -> BitBoard {
    let open_cells = !(player | opponent);
    let mut result = BitBoard::empty();

    for direction in DirectionIterator::new() {
        result |= match_pattern_base(
            player,
            opponent,
            pattern,
            pattern_size,
            move_step,
            closure_bits,
            open_cells,
            direction
        );
    }

    result
}

pub fn match_pattern_all_axis(
    player: BitBoard,
    opponent: BitBoard,
    pattern: u8,
    pattern_size: u8,
    move_step: u8,
    closure_bits: u8
) -> BitBoard {
    let open_cells = !(player | opponent);
    let mut result = BitBoard::empty();

    for direction in AxisIterator::new() {
        result |= match_pattern_base(
            player,
            opponent,
            pattern,
            pattern_size,
            move_step,
            closure_bits,
            open_cells,
            direction
        );
    }

    result
}

// pub fn match_pattern_all_directions(
//     player: BitBoard,
//     opponent: BitBoard,
//     pattern: u8,
//     pattern_size: u8,
//     move_step: u8,
//     closure_bits: u8
// ) -> BitBoard {
//     let open_cells = !(player | opponent);
//     let mut result = BitBoard::empty();

//     for direction in DirectionIterator::new() {
//         let mut tmp = if closure_bits == U8_FIRST_BIT { opponent } else { BitBoard::full() };
//         for x in 0..pattern_size {
//             if tmp.is_empty() {
//                 break;
//             }
//             tmp = (tmp >> direction) & if (pattern << x) & U8_FIRST_BIT == U8_FIRST_BIT {
//                 player
//             } else {
//                 open_cells
//             };
//         }
//         result |= tmp.shift_direction_by(direction.to_invert(), move_step);
//     }

//     result
// }

pub fn match_pattern(player: BitBoard, opponent: BitBoard, pattern: u8, pattern_size: u8, is_pattern_symmetric: bool) -> BitBoard {
    let closure_bits = (pattern & U8_FIRST_BIT) | (1 << (8 - pattern_size) & pattern);
    if is_pattern_symmetric {
        match_pattern_all_axis(player, opponent, pattern, pattern_size, 0, closure_bits)
    } else {
        match_pattern_all_directions(player, opponent, pattern, pattern_size, 0, closure_bits)
    }
}

pub fn extract_five_aligned(player: BitBoard) -> BitBoard {
    let mut result = BitBoard::empty();

    for direction in AxisIterator::new() {
        let mut tmp = player;
        let mut i = 0;
        while i < 4 && tmp.is_any() {
            tmp = (tmp << direction) & player;
            i += 1;
        }
        if tmp.is_empty() {
            continue;
        }
        for i in 0..5 {
            result |= tmp.shift_direction_by(direction.to_invert(), i);
        }
    }

    result
}

// TODO: Investiguate and fix this!
pub fn extract_illegal_moves(player: BitBoard, opponent: BitBoard, patterns: NewPattern) -> BitBoard {
    let open_cells = !(player | opponent);
    let illegal_patterns = [
        patterns[PatternName::OpenSplitThreeLeft],
        patterns[PatternName::OpenSplitThreeRight],
        patterns[PatternName::OpenThree]
    ];
    let mut result = BitBoard::empty();
    let mut tmp = [
        BitBoard::empty(),
        BitBoard::empty(),
        BitBoard::empty(),
        BitBoard::empty(),
    ];

    for &(pattern, pattern_size, _) in illegal_patterns.iter() {
        for i in 0..pattern_size {
            let sub_pattern = pattern & !(U8_FIRST_BIT >> i);
            if sub_pattern == pattern {
                continue;
            }
            for (id, direction) in AxisIterator::new().enumerate() {
                tmp[id] |= match_pattern_base(player, opponent, sub_pattern, pattern_size, pattern_size - i - 1, 0, open_cells, direction) & open_cells;
                if tmp[id].is_empty() {
                    continue;
                }
                for iid in (0..id).rev() {
                    result |= tmp[id] & tmp[iid];
                }
            }
        }
    }

    result
}

pub fn extract_threatening_moves_from_opponent(player: BitBoard, opponent: BitBoard, pattern: u8, pattern_size: u8, is_pattern_symmetric: bool) -> BitBoard {
    let closure_bits = (pattern & U8_FIRST_BIT) | ((U8_FIRST_BIT >> (pattern_size - 1)) & pattern);
    let open_cells = !(player | opponent);
    let directions = if is_pattern_symmetric { AxisIterator::as_array_iter() } else { DirectionIterator::as_array_iter() };
    let mut result = BitBoard::empty();

    for &direction in directions {
        let mut tmp = match_pattern_base(opponent, player, pattern, pattern_size, 0, closure_bits, open_cells, direction);
        if tmp.is_empty() {
            continue;
        }
        for _ in 0..(pattern_size - 1) {
            tmp |= tmp << direction.to_invert();
        }
        result |= tmp & open_cells;
    }

    result
}

// FIXME: It seems that this fonction doesn't concider the edges as occupied places
// thus the patterns for close (CloseThree, CloseSplitThreeLeft, CloseSplitThreeRight & CloseFour)
// will not match when the close side is right next to an edge (it wouldn't match either when the open side is
// right next to an edge, even is the close side is next to an opponent stone).
// Take a look at the `test_pattern_matching_extract_missing_bit_with_close_three()` test function for more detail.
/// Returns the bits that are missing to complete the provided pattern.
/// Only one bit by potential match are returned, understand that the returned bits
/// are the ones we can play to complete the provided pattern in one move.
pub fn extract_missing_bit(player: BitBoard, opponent: BitBoard, pattern: u8, pattern_size: u8, is_sym: bool) -> BitBoard {
    let closure_bits = (pattern & U8_FIRST_BIT) | (pattern & (U8_FIRST_BIT >> (pattern_size - 1)));
    let mut result = BitBoard::empty();

    for i in 0..pattern_size {
        let tmp = pattern & !(U8_FIRST_BIT >> i);
        if tmp == pattern {
            continue;
        }
        if is_sym {
            result |= match_pattern_all_axis(player, opponent, tmp, pattern_size, pattern_size - i - 1, closure_bits);
        } else {
            result |= match_pattern_all_directions(player, opponent, tmp, pattern_size, pattern_size - i - 1, closure_bits);
        }
    }

    result & !(player | opponent)
}

pub fn extract_captured_by_move(player: BitBoard, opponent: BitBoard, being_played: BitBoard) -> BitBoard {
    let mut result = BitBoard::empty();

    for direction in DirectionIterator::new() {
        let mut tmp = being_played;
        let mut i = 0;
        while i < 3 && tmp.is_any() {
            tmp = (tmp >> direction) & if (U8_TWO_FIRST_BITS << i) & U8_FIRST_BIT == U8_FIRST_BIT { opponent } else { player };
            i += 1;
        }
        if tmp.is_any() {
            let inverted_direction = direction.to_invert();
            result |= tmp.shift_direction_by(inverted_direction, 2) | tmp.shift_direction_by(inverted_direction, 1);
        }
    }

    result
}

pub fn contains_five_aligned(player: BitBoard) -> bool {
    for direction in DirectionIterator::new() {
        let mut tmp = player;
        let mut i = 1;
        while tmp.is_any() {
            tmp &= tmp << direction;
            if i >= 5 {
                return true;
            }
            i += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_index() {
        // Arrange
        let expected: [(u8, u8, bool); 13] = [
            (0b11000000, 3, false),
            (0b01110000, 5, true),
            (0b01101000, 6, false),
            (0b01011000, 6, false),
            (0b01111000, 6, true),
            (0b11100000, 4, false),
            (0b11010000, 5, false),
            (0b10110000, 5, false),
            (0b11110000, 5, false),
            (0b01011100, 6, false),
            (0b01101100, 6, false),
            (0b01110100, 6, false),
            (0b11111000, 5, true)
        ];
        let patterns = NewPattern::new();

        // Act
        let results: [(u8, u8, bool); 13] = [
            patterns[PatternName::CloseTwo],
            patterns[PatternName::OpenThree],
            patterns[PatternName::OpenSplitThreeRight],
            patterns[PatternName::OpenSplitThreeLeft],
            patterns[PatternName::OpenFour],
            patterns[PatternName::CloseThree],
            patterns[PatternName::CloseSplitThreeRight],
            patterns[PatternName::CloseSplitThreeLeft],
            patterns[PatternName::CloseFour],
            patterns[PatternName::SplitFourLeft],
            patterns[PatternName::SplitFourMiddle],
            patterns[PatternName::SplitFourRight],
            patterns[PatternName::Five]
        ];

        // Assert
        assert_eq!(expected, results);
    }

    #[test]
    fn test_pattern_matching_contains_five_aligned() {
        // Arrange
        let one_five_aligned = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000000000
            0000000000000011000
            0000000000000100000
            0000000000001001000
            0000000000010000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001110110000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let no_five_aligned = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000001000
            0000001000000001000
            0000000100000000000
            0000000000000001000
            0000000001000000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001110110000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let two_five_aligned = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000000000
            0000000000000011000
            0000000000000100000
            0000000000001001000
            0000000000010000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001111100000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let one_six_aligned = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000000000
            0000000000000011000
            0000000000000000000
            0000000000001001000
            0000000000010000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001111110000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let one_seven_aligned = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000000000
            0000000000000011000
            0000000000000100000
            0000000000000001000
            0000000000010000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000011111110000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let one_five_aligned_edge = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000000000
            0000000000000011000
            0000000000000100000
            0000000000000001000
            0000000000010000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000011001100010000
            0000000000000001000
            0000000000000000100
            0000000000000000010
            0000000000000000001
        ");
        let expected = [true, false, true, true, true, true];

        // Act
        let results = [
            contains_five_aligned(one_five_aligned),
            contains_five_aligned(no_five_aligned),
            contains_five_aligned(two_five_aligned),
            contains_five_aligned(one_six_aligned),
            contains_five_aligned(one_seven_aligned),
            contains_five_aligned(one_five_aligned_edge)
        ];

        // Assert
        assert_eq!(expected, results);
    }

    #[test]
    fn test_pattern_matching_with_split_three() {
        // Arrange
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::OpenSplitThreeLeft];
        let player = BitBoard::from_str("
            1000000100000010000
            0100000010000001000
            0010000001000000100
            0001000000000000010
            0000100000001100001
            0000010000000100000
            0000001000001000000
            0000000100001100000
            0000000010000010000
            0000000001000001000
            0010111000100000100
            0000000000010000010
            0000000000001000001
            0000001000000100000
            0000000100000010000
            0000000010000001000
            0000000000000000100
            1000000000100000010
            0100000000000000001
        ");
        let opponent = BitBoard::empty();
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000100000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000001000000
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
        ");

        // Act
        let result = match_pattern(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_with_split_four() {
        // Arrange
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::SplitFourRight];
        let player = BitBoard::from_str("
            1000000100000010000
            0100000010000001000
            0010000001000000100
            0001000000001100010
            0000100000000100001
            0000010000001100000
            0000001000001000000
            0000000100001100000
            0000000010000010000
            0000000001000001000
            0010111000100000100
            0000000000010000010
            0000000000001000001
            0000001000000100000
            0000000100000010000
            0000000010000001000
            0000000000000000100
            1000000000100000010
            0100000000000000001
        ");
        let opponent = BitBoard::empty();
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000001000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000100000
            0000000000000000000
            0000000000000000000
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000100000000
            0000000000000000000
        ");

        // Act
        let result = match_pattern(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_five_aligned() {
        // Arrange
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::Five];
        let player = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000001000
            0000001000000001000
            0000000100000000000
            0000000010000001000
            0000000001000000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001110110000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let opponent = BitBoard::empty();
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000001000000000000
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
        ");

        // Act
        let result = match_pattern(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_isolate_five_aligned() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000001000
            0000001000000001000
            0000000100000000000
            0000000010000001000
            0000000001000000000
            0000000000100000000
            0000000000000000000
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001110110000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000001000000000000
            0000000100000000000
            0000000010000000000
            0000000001000000000
            0000000000100000000
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
        ");

        // Act
        let result = extract_five_aligned(player);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_open_three() {
        // Arrange
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::OpenThree];
        let player = BitBoard::from_str("
            0000000000000001000
            0000000000000001000
            0000000000000001000
            0000001000000001000
            0000000100000000000
            0000000010000001000
            0000000001000001000
            0000000000100000100
            0000000000000000010
            0100100010010100000
            0000000000000000000
            0000000000000100000
            0000000000001010000
            0000000000000000000
            0000001110110000000
            0001010000001001000
            0010100000010010100
            0100000000000100010
            0000000000000000000
        ");
        let opponent = BitBoard::from_str("
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
            0000000000000000100
            0000000001000001010
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000010100000
        ");
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000010000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000100000000000
            0000100000000010100
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");

        // Act
        let result = match_pattern(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_check_illegal_moves() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            0000000110000000000
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
        ");
        let opponent = BitBoard::from_str("
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
            0000000000000000000
            0000000000000000000
        ");
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
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
        ");
        let patterns = NewPattern::new();

        // Act
        let result = extract_illegal_moves(player, opponent, patterns);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_check_illegal_moves_with_no_illegal() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            0000000110000000000
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
        ");
        let opponent = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
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
        ");
        let expected = BitBoard::from_str("
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
            0000000000000000000
            0000000000000000000
        ");
        let patterns = NewPattern::new();

        // Act
        let result = extract_illegal_moves(player, opponent, patterns);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_extract_threatening_moves_from_opponent_with_open_three() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            0000000110000000000
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
        ");
        let opponent = BitBoard::from_str("
            0001000001110000000
            0010000000000000000
            0100000000000000000
            0000000000000000000
            0000000000000001110
            0000000000000000000
            0000000000000000000
            0000000001000000001
            1110000000000000001
            0000000000000000001
            0000000000000000000
            1000000000111000000
            1000000000000000000
            1000000000000000000
            0000000000000000000
            0001000000100001000
            0010000000100000100
            0100000000100000010
            0001110000000000000
        ");
        let expected = BitBoard::from_str("
            0000000010001000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000010001
            0000000000000000000
            0000000000000000001
            0000000000000000000
            0000000000000000000
            0000000000000000000
            1000000000000000001
            0000000001000100000
            0000000000000000000
            0000000000000000000
            1000100000100010000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            1010001000100000001
        ");
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::OpenThree];

        // Act
        let result = extract_threatening_moves_from_opponent(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_extract_missing_bit_with_open_three() {
        // Arrange
        let player = BitBoard::from_str("
            0000000010011000001
            0100000010000000010
            0010000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            1000000110000000000
            1000000000000000000
            0000000000000000011
            0000000000000000000
            0110000000000000000
            0000000000000000001
            0000000000000000001
            0000000000000000000
            0000000000000001000
            0010000001000000100
            0100000001000000000
            0000000000000110000
        ");
        let opponent = BitBoard::empty();
        let expected = BitBoard::from_str("
            0000000000100100000
            0000000000000000000
            0000000000000000000
            0001000000000000000
            0000000001000000000
            0000000000100000000
            1000000010000000000
            0000001001000000000
            0000000100000000000
            1000000000000000000
            0000000000000000000
            0001000000000000001
            0000000000000000000
            0000000000000000000
            0000000000000010001
            0001000001000000000
            0000000000000000000
            0000000000000000010
            0000000000001001000
        ");
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::OpenThree];

        // Act
        let result = extract_missing_bit(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_extract_missing_bit_with_close_three() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000100000
            0000000010000000000
            0010000000000000000
            0000000010000000100
            0000100000000000000
            0000000000000000000
            0000000000000011000
            1000000011000000000
            0000000000000000000
            0000000000000000011
            1000000000000000000
            0000000001000000000
            0000000000000000000
            0000000001000000000
            0000000000000000000
            0001000000000001000
            0000000000000000100
            0000000000000000000
            1000000000000000000
        ");
        let opponent = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000010000000000
            0000000000000000000
            0000000000000000001
            0000010000000000000
            0000000000000000000
            0000000000000010000
            0000000000000000000
            1000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000010
            0000001000000000000
            1000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let (pattern, pattern_size, is_sym) = NewPattern::new()[PatternName::CloseSplitThreeLeft];

        // Act
        let result = extract_missing_bit(player, opponent, pattern, pattern_size, is_sym);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_extract_captured_by_move() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000000000000
            0000000000000000000
            0000001001000000000
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
        let player_last_move = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
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
        let opponent = BitBoard::from_str("
            0000000000000000000
            0000001100000000000
            0000000000000000000
            0000000000000100000
            0000000000000100000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            0000000110000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000010000000000
            0000100000000010000
            0000100000000001000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            0000000110000000000
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

        // Act
        let result = extract_captured_by_move(player, opponent, player_last_move);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pattern_matching_extract_captured_by_move_in_edge() {
        // Arrange
        let player = BitBoard::from_str("
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
            1000000000000000000
            0000000000000000000
            0000000000000000000
            1001000000000000000
        ");
        let player_last_move = BitBoard::from_str("
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
            0000000000000000000
            1000000000000000000
        ");
        let opponent = BitBoard::from_str("
            0000000000000000000
            0000001100000000000
            0000000000000000000
            0000000000000100000
            0000000000000100000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000001000000000
            0000000110000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000010000000000
            0000100000000010000
            0000100000000001000
            1000000000000000000
            1000000000000000000
            0110000000000000000
        ");
        let expected = BitBoard::from_str("
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
            1000000000000000000
            1000000000000000000
            0110000000000000000
        ");

        // Act
        let result = extract_captured_by_move(player, opponent, player_last_move);

        // Assert
        assert_eq!(expected, result);
    }
}