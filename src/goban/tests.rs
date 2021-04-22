// use crate::bitboard::BitBoard;
// use crate::goban::Goban;

// use crate::goban::fscore::Fscore;

// #[test]
// fn test_goban_neighbour_layers_simple()
// {
//     let to_play =
//         BitBoard::from_str("
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000010000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     ");

//     let player =
//         BitBoard::from_str("
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000010000000000
//     0000000010000000000
//     0000000001000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     ");

//     let enemy =
//         BitBoard::from_str("
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000010000000
//     0000000000100000000
//     0000000001000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     ");

//     let board = Goban::new(player, enemy);

//     println!("{}", to_play);
//     assert_eq!(4, board.neighbour_layering(&to_play));
// }

// #[test]
// fn test_goban_neighbours_simple()
// {
//     let player =
//         BitBoard::from_str("
//     0000000000000000001
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0001000000000000000
//     0001000000000000000
//     0001000000000011111
//     0001000000000111110
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     ");

//     let enemy =
//         BitBoard::from_str("
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000110000000
//     0000000001100000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000001111100000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0001110000000000000
//     1100000000000000000
//     ");

//     let expected =
//         BitBoard::from_str("
//     0000000000000000010
//     0000000000000000011
//     0000000001111000000
//     0000000011001000000
//     0000000010011000000
//     0000000011110000000
//     0011100000000000000
//     0010100000000000000
//     0010100000000111111
//     0010100000001100000
//     0010100000001000001
//     0011100000001111111
//     0000011111110000000
//     0000010000010000000
//     0000011111110000000
//     0000000000000000000
//     0011111000000000000
//     1110001000000000000
//     0011111000000000000
//     ");
//     let board = Goban::new(player, enemy);
//     println!("PLAYER\n{}\nENEMY\n{}\nFULL\n{}", player, enemy, player | enemy);
//     println!("RESULT\n{}\nEXPECTED\n{}", board.list_neighbours(), expected);
//     assert_eq!(board.list_neighbours(), expected);
// }

// #[test]
// fn test_goban_neighbours_borders()
// {
//     let player =
//         BitBoard::from_str("
//     1111111111111111111
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1000000000000000001
//     1111111111111111111
//     ");

//     let expect =
//         BitBoard::from_str("
//     0000000000000000000
//     0111111111111111110
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0100000000000000010
//     0111111111111111110
//     0000000000000000000
//     ");

//     let board = Goban::new(player, BitBoard::default());
//     println!("RESULT\n{}\nEXPECTED\n{}", board.list_neighbours(), expect);
//     assert_eq!(board.list_neighbours(), expect);
// }

// #[test]
// fn test_goban_alignment_simple()
// {
//     let original =
//         BitBoard::from_str("
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000010000000
//     0000000000100000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000001111000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0001110000000000000
//     0000000000000000000
//     ");

//     let board = Goban::new(original, BitBoard::empty());
//     // println!("HSCORE= {}", board.line_detection());
//     assert_eq!(Fscore::Value(6), board.line_detection());
// }

// #[test]
// fn test_goban_alignment_win()
// {
//     let original =
//         BitBoard::from_str("
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000001000000000000
//     0000001000000000000
//     0000001000000000000
//     0000001000000000000
//     0000001000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     ");
//     let board = Goban::new(original, BitBoard::empty());
//     assert_eq!(Fscore::Win, board.line_detection());
// }

// #[test]
// fn test_goban_alignment_wraparound()
// {

//     let original =
//         BitBoard::from_str("
//     0100000010000000000
//     1000000010000000000
//     0000000000000000001
//     0000000000000000010
//     0000000000000000000
//     0000000000000000010
//     0000000000000000001
//     1000000000000000000
//     0100000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000011
//     1100000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000000000000000
//     0000000010000000000
//     0000000010000000000
//     ");

//     let board = Goban::new(original, BitBoard::empty());
//     println!("HSCORE= {}", board.line_detection());
//     assert_eq!(Fscore::Value(8), board.line_detection());
// }