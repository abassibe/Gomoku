use std::{
    rc::Rc,
    cell::RefCell,
    collections::{BinaryHeap}
};

use crate::{bitboard::BitBoard, goban::Goban};
use super::{Node, Branches};

#[test]
#[allow(unused)]
fn add_many_branches_with_valid_node_generator_should_add_branches()
{
    // Arrange
    let closure = |n: &mut Node, m| {
        let mut vec = Vec::new();
        for i in 1..10 {
            let mut bitboard = BitBoard::default();
            bitboard.set(i, true);
            vec.push(Node::new(Goban::new(bitboard, bitboard), n.depth + 1, BitBoard::empty(), false,0, 0));
        }
        vec
    };
    let mut node = Node::new(Goban::default(), 0, BitBoard::empty(), false,0, 0);
    let new_branches = closure(&mut node, true);

    // Act
    node.add_many_branches(new_branches);
    let nb_branches = node.count_branch();

    // Assert
    assert_eq!(nb_branches, 9);
}

#[test]
fn count_branch_with_0_branch_should_return_0() {
    // Arrange
    let bitboard = BitBoard::default();
    let node = Node {
        item: Goban::new(bitboard, bitboard),
        depth: 0,
        last_move: BitBoard::empty(),
        is_players_move: false,
        player_captures: 0,
        opponent_captures: 0,
        is_player_threatened: Some(false),
        branches: Some(Branches::new())
    };

    // Act
    let nb_branches = node.count_branch();

    // Assert
    assert_eq!(nb_branches, 0);
}

#[test]
fn count_branch_with_no_branch_should_return_0() {
    // Arrange
    let bitboard = BitBoard::default();
    let node = Node {
        item: Goban::new(bitboard, bitboard),
        depth: 0,
        last_move: BitBoard::empty(),
        is_players_move: false,
        player_captures: 0,
        opponent_captures: 0,
        is_player_threatened: Some(false),
        branches: None
    };

    // Act
    let nb_branches = node.count_branch();

    // Assert
    assert_eq!(nb_branches, 0);
}

#[test]
fn count_branch_with_3_branches_should_return_3() {
    // Arrange
    let bitboards = [BitBoard::full(), BitBoard::empty()];
    let (node0, node1, node2) = (
        Rc::new(RefCell::new(Node {
            item: Goban::new(bitboards[0], bitboards[1]),
            depth: 1,
            last_move: BitBoard::empty(),
            is_players_move: false,
            player_captures: 0,
            opponent_captures: 0,
            is_player_threatened: Some(false),
            branches: None
        })),
        Rc::new(RefCell::new(Node {
            item: Goban::new(bitboards[1], bitboards[0]),
            depth: 1,
            last_move: BitBoard::empty(),
            is_players_move: false,
            player_captures: 0,
            opponent_captures: 0,
            is_player_threatened: Some(false),
            branches: None
        })),
        Rc::new(RefCell::new(Node {
            item: Goban::new(bitboards[0], bitboards[0]),
            depth: 1,
            last_move: BitBoard::empty(),
            is_players_move: false,
            player_captures: 0,
            opponent_captures: 0,
            is_player_threatened: Some(false),
            branches: None
        }))
    );
    let mut branches = BinaryHeap::new();
    branches.push(node0);
    branches.push(node1);
    branches.push(node2);
    let node = Node {
        item: Goban::new(bitboards[1], bitboards[1]),
        depth: 0,
        last_move: BitBoard::empty(),
        is_players_move: false,
        player_captures: 0,
        opponent_captures: 0,
        is_player_threatened: Some(false),
        branches: Some(branches)
    };

    // Act
    let nb_branches = node.count_branch();

    // Assert
    assert_eq!(nb_branches, 3);
}

#[test]
fn add_branch_should_add_a_branch() {
    // Arrange
    let bitboards = [BitBoard::full(), BitBoard::empty()];
    let mut node = Node {
        item: Goban::new(bitboards[0], bitboards[1]),
        depth: 0,
        last_move: BitBoard::empty(),
        is_players_move: false,
        player_captures: 0,
        opponent_captures: 0,
        is_player_threatened: Some(false),
        branches: None
    };

    // Act
    let new_node = node.add_branch(Goban::new(bitboards[1], bitboards[0]), BitBoard::empty(), false);
    let nb_branches = node.count_branch();

    // Assert
    assert_eq!(nb_branches, 1);
    assert_eq!(node.branches.unwrap().peek().unwrap(), &new_node);
}

#[test]
#[ignore]
fn test_display_no_assert() {
    // Arrange
    let closure = |n: &mut Node, m| {
        let mut vec = Vec::new();
        for i in 1..10 {
            let mut bitboard = BitBoard::default();
            bitboard.set(i, true);
            vec.push(Node::new(Goban::new(bitboard, bitboard), n.depth + 1, BitBoard::empty(), false,0, 0));
        }
        vec
    };
    let mut node = Node::new(Goban::default(), 0, BitBoard::empty(), false, 0, 0);
    let new_branches = closure(&mut node, true);

    // Act
    node.add_many_branches(new_branches);
    println!("Here is a node with 9 branches:\n{}", node);
}