use super::{*, axis::*, direction::*};

// Tests for struct BitBoard
#[test]
fn test_bitshift_left_by_4_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::from_array([
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000101,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000
    ]);

    // Act
    let result = original.shift_left(4);

    // Assert
    assert_eq!(expected.b, result.b);
}

#[test]
fn test_bitshift_left_by_128_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::from_array([
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    ]);

    // Act
    let result = original.shift_left(128);

    // Assert
    assert_eq!(expected.b, result.b);
}

#[test]
fn test_bitshift_left_by_0_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);

    // Act
    let result = original.shift_left(0);

    // Assert
    assert_eq!(original.b, result.b);
}

#[test]
fn test_bitshift_left_by_max_minus_1_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::from_array([
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    ]);
    let by = (BITS_IN_U128 * 3) - 1;

    // Act
    let result = original.shift_left(by);

    // Assert
    assert_eq!(expected.b, result.b);
}

#[test]
fn test_bitshift_left_by_max_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::default();
    let by = BITS_IN_U128 * 3;

    // Act
    let result = original.shift_left(by);

    // Assert
    assert_eq!(expected.b, result.b);
}

// The following test perform no assertion because we just want to verify
// that no panic occurs regardeless the value passed to the function.
#[test]
fn test_bitshift_left_by_any_value() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);

    // Act
    for x in 0..(size_of::<u128>() * 3) {
        original.shift_left(x);
    }

    // No assert
}

#[test]
fn test_bitshift_right_by_4_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011010,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010011
    ]);
    let expected = BitBoard::from_array([
        0b00001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b00010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001,
        0b10100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);

    // Act
    let result = original.shift_right(4);

    // Assert
    assert_eq!(expected.b, result.b);
}

#[test]
fn test_bitshift_right_by_128_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::from_array([
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    ]);

    // Act
    let result = original.shift_right(128);

    // Assert
    assert_eq!(expected.b, result.b);
}

#[test]
fn test_bitshift_right_by_0_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);

    // Act
    let result = original.shift_right(0);

    // Assert
    assert_eq!(original.b, result.b);
}

#[test]
fn test_bitshift_right_by_max_minus_1_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::from_array([
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let by = (BITS_IN_U128 * 3) - 1;

    // Act
    let result = original.shift_right(by);

    // Assert
    assert_eq!(expected.b, result.b);
}

#[test]
fn test_bitshift_right_by_max_on_bitboard() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);
    let expected = BitBoard::default();
    let by = BITS_IN_U128 * 3;

    // Act
    let result = original.shift_right(by);

    // Assert
    assert_eq!(expected.b, result.b);
}

// The following test perform no assertion because we just want to verify
// that no panic occurs regardeless the value passed to the function.
#[test]
fn test_bitshift_right_by_any_value() {
    // Arrange
    let original = BitBoard::from_array([
        0b11001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b01011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
        0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001
    ]);

    // Act
    for x in 0..(size_of::<u128>() * 3) {
        original.shift_right(x);
    }

    // No assert
}

// Tests for struct AxisIterator
#[test]
fn test_iterate_on_axis_iterator() {
    // Arrange
    let axises = AxisIterator::new();
    let expect = vec![Direction::W, Direction::N, Direction::NW, Direction::NE];

    // Act
    let result: Vec<Direction> = axises.collect();

    // Assert
    assert_eq!(expect, result);
}

// Tests for struct DirectionIterator
#[test]
fn test_iterate_on_direction_iterator() {
    // Arrange
    let expect = vec![Direction::N, Direction::S, Direction::E, Direction::W, Direction::NE, Direction::NW, Direction::SE, Direction::SW];

    // Act
    let result: Vec<&Direction> = directions.collect();

    // Assert
    assert_eq!(expect, result);
}

// TODO: Add tests for Not (trait implementation)
// TODO: Add tests for BitOr (trait implementation)
// TODO: Add tests for BitXor (trait implementation)
// TODO: Add tests for BitAnd (trait implementation)
// TODO: Add tests for Eq/PartialEq (derived trait)