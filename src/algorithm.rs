use std::rc::Rc;

use crate::bitboard::{direction::Direction, new_pattern::*};

use super::{
    node::Node,
    goban::{Goban, Fscore},
    bitboard::BitBoard
};

#[derive(Default)]
pub struct Algorithm
{
    initial: Node,
    patterns: NewPattern,
    player_captures: u8,
    opponent_captures: u8
}

impl Algorithm
{
    /// Initialize a new default Algorithm object.
    pub fn new() -> Self {
        Algorithm::default()
    }

    /// Set the initial Node to a new state using the provided Goban.
    pub fn update_initial_state(&mut self, initial_state: Goban, last_move: BitBoard, player_captures: u8, opponent_captures:u8) {
        let new_initial_node = Node::new(initial_state, 0, last_move, player_captures, opponent_captures);
        self.initial = new_initial_node;
    }

    // TODO: There is a lot of duplicated code in this function, we should refactor it.
    fn minimax(node: &mut Node, depth: u32, mut alpha: Fscore, mut beta: Fscore, maximizing: bool) -> Node {
        let current_goban = node.get_item().clone();
        if depth == 0 {
            // TODO: We have to passe the potential next move to compute_item_fscore but we don't have it at this point
            // and I'm not even sure we actually need it, maybe we should remove it completely?
            node.compute_item_fscore(&current_goban, current_goban.get_player(), depth as usize);
            return node.clone();
        }
        let mut candidate = node.clone();
        let mut fscore = node.get_item().get_fscore();
        if fscore.is_win() {
            return candidate;
        }

        if maximizing {
            fscore = Fscore::Value(isize::MIN);
            node.add_many_branches(Self::node_generator, maximizing);
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild = Self::minimax(&mut child.borrow_mut(), depth - 1, alpha, beta, !maximizing);
                    let grandchild_fscore = grandchild.get_item().get_fscore();
                    child.borrow_mut().set_item_fscore(grandchild_fscore);
                    if fscore < grandchild_fscore {
                        candidate = child.borrow().clone();
                        fscore = grandchild_fscore;
                    }
                    alpha = alpha.max(grandchild_fscore);
                    if beta <= alpha {
                        break;
                    }
                }
            }
        }
        else {
            fscore = Fscore::Value(isize::MAX);
            node.add_many_branches(Self::node_generator, maximizing);
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild = Self::minimax(&mut child.borrow_mut(), depth - 1, alpha, beta, !maximizing);
                    let grandchild_fscore = grandchild.get_item().get_fscore();
                    child.borrow_mut().set_item_fscore(grandchild_fscore);
                    if fscore > grandchild_fscore {
                        candidate = child.borrow().clone();
                        fscore = grandchild_fscore;
                    }
                    beta = beta.min(grandchild_fscore);
                    if beta <= alpha {
                        break;
                    }
                }
            }
        }

        candidate
    }

    fn node_generator(parent: &mut Node, maximazing: bool) -> Vec<Node> {
        parent
            .get_item()
            .list_neighbours()
            .enumerate()
            .iter()
            .map(|b| {
                let (player, enemy) =
                if maximazing {
                    (parent.get_item().get_player() | b, *parent.get_item().get_enemy())
                } else {
                    (*parent.get_item().get_player(), parent.get_item().get_enemy() | b)
                };
                Node::new(Goban::new(player, enemy), parent.get_depth() + 1, *b, 0, 0)
            })
            .collect()
    }

    #[inline]
    fn get_first_move(player: BitBoard, opponent: BitBoard) -> BitBoard {
        match (player.is_empty(), opponent.is_empty()) {
            (true, false) => (opponent + Direction::All) & !player,
            (true, true) => BitBoard::CENTER_BIT_SET,
            (false, _) => BitBoard::empty()
        }
    }

    #[inline]
    fn counter_five_aligned(&self, player: BitBoard, opponent: BitBoard, player_captures: u8) -> BitBoard {
        let mut result = extract_five_align_breaking_moves(player, opponent, &self.patterns);

        if result.is_empty() {
            extract_winning_move_capture(player, opponent, player_captures, &self.patterns)
        } else {
            result
        }
    }

    #[inline]
    fn compute_illegal_moves(&self) -> BitBoard {
        let goban = self.initial.get_item();
        let player = *goban.get_player();
        let opponent = *goban.get_enemy();

        extract_illegal_moves(player, opponent, &self.patterns)
    }

    fn get_potential_moves(&self) -> BitBoard {
        let goban = self.initial.get_item();
        let player = *goban.get_player();
        let opponent = *goban.get_enemy();
        let player_captures = self.player_captures;
        let opponent_captures = self.opponent_captures;
        let open_cells = !(player | opponent);
        let illegal_moves_complement = !self.compute_illegal_moves();

        if player.is_empty() {
            return open_cells & Self::get_first_move(player, opponent);
        }

        if opponent.contains_five_aligned() {
            let result = self.counter_five_aligned(player, opponent, player_captures);
            if result.is_any() {
                return result & open_cells;
            }
        }

        let result = extract_winning_moves_from_player(player, opponent, player_captures, opponent_captures, &self.patterns);
        if result.is_any() {
            return result;
        }

        let result = extract_winning_moves_from_player(opponent, player, opponent_captures, player_captures, &self.patterns);
        if result.is_any() {
            let (pattern, pattern_size, is_sym) = self.patterns[PatternName::Five];
            return result | extract_missing_bit(player, opponent, pattern, pattern_size, is_sym);
        }

        let mut result = extract_threatening_moves_from_player(player, opponent, opponent_captures, &self.patterns);
        result |= extract_capturing_moves(opponent, player, &self.patterns);
        result |= extract_missing_bit(player, opponent, GET_MOVES_PATTERNS[0].0, GET_MOVES_PATTERNS[0].1, GET_MOVES_PATTERNS[0].2);
        result |= extract_missing_bit(player, opponent, GET_MOVES_PATTERNS[1].0, GET_MOVES_PATTERNS[1].1, GET_MOVES_PATTERNS[1].2);
        result |= extract_capturing_moves(player, opponent, &self.patterns);

        if result.count_ones() > 4 {
            return result & open_cells & illegal_moves_complement;
        }

        result |= extract_threatening_moves_from_opponent(player, opponent, GET_MOVES_PATTERNS[2].0, GET_MOVES_PATTERNS[2].1, GET_MOVES_PATTERNS[2].2);
        result |= extract_missing_bit(player, opponent, GET_MOVES_PATTERNS[3].0, GET_MOVES_PATTERNS[3].1, GET_MOVES_PATTERNS[3].2);
        result |= extract_missing_bit(player, opponent, GET_MOVES_PATTERNS[4].0, GET_MOVES_PATTERNS[4].1, GET_MOVES_PATTERNS[4].2);

        if result.count_ones() > 4 {
            return result & open_cells & illegal_moves_complement;
        }

        result |= extract_missing_bit(player, opponent, GET_MOVES_PATTERNS[5].0, GET_MOVES_PATTERNS[5].1, GET_MOVES_PATTERNS[5].2);

        if result.count_ones() > 2 {
            return result & open_cells & illegal_moves_complement;
        }

        (result | (player + Direction::All)) & open_cells & illegal_moves_complement
    }

    // fn set_with_capture(state: BitBoard, to_play: &BitBoard) -> BitBoard {
    //     let next_state = 
    // }

    /// This mehtod is likely to change in a near future because I'm not sure what to return.
    /// For now it returns a BitBoard that contains the next move to play.
    pub fn get_next_move(&mut self) -> Option<Node> {
        let next_state = Self::minimax(&mut self.initial, 1, Fscore::MIN, Fscore::MAX, true);
        if next_state == self.initial {
            None
        } else {
            Some(next_state)
            // Some(next_state.get_item().get_player() ^ self.initial.get_item().get_player())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{goban::Goban, tree::node::Node};
    use crate::bitboard::BitBoard;
    use crate::algorithm::Algorithm;

    #[test]
    fn test_get_potential_moves_with_one_unbreakable_five() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001010000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000100000000
            0000000000010000000
            0000000100000100000
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
            0000000000001000000
            0000000000010000000
            0000001110100000000
            0000000001000000000
            0000000010000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000011011000000
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
            0000000000000100000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000100000000000
            0000000000000000000
            0000000000000000000
            0000000000100000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_potential_moves_with_one_breakable_five() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000100000000000
            0000000001010000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000100000000
            0000000000010000000
            0000000100000100000
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
            0000000000001000000
            0000000000010000000
            0000001110100000000
            0000000001000000000
            0000000010000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000011011000000
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
        ");
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_potential_moves_with_multiple_threats_and_one_split_four() {
        // Arrange
        let player = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001010000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000100000000
            0000000000010000000
            0000000100000100000
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
            0000001110100000000
            0000000001000000000
            0000000010000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000011011000000
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
            0000000000100000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_potential_moves_with_threat_from_opponent() {
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
            0000000001000000000
            0000000000100000000
            0000000000010000000
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
            0000000000100000000
            0000000001000000000
            0000000010000000000
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
            0000000000010000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000100000000000
            0000000000000000000
            0000000000000000000
            0000000000001000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_potential_moves_with_first_turn_completed() {
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
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000010000000000
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
            0000000001100000000
            0000000010100000000
            0000000011100000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_potential_moves_with_no_move_played() {
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
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_potential_moves_with_only_one_opponent_move() {
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
        let expected = BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000011100000000
            0000000010100000000
            0000000011100000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        ");
        let mut algo = Algorithm::new();
        algo.update_initial_state(Goban::new(player, opponent), BitBoard::empty(), 0, 0);

        // Act
        let result = algo.get_potential_moves();
        println!("Here is the result:\n{}", result);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    // This test is quite time-consuming and serves basically no purpose at this point,
    // we better not to run it especially using GitHub Actions.
    #[ignore]
    fn test_algorithm()
    {
        let (mut player, mut enemy) = (BitBoard::default(), BitBoard::from_str("
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
        "));
        let mut next_move = enemy;
        let mut algo = Algorithm::new();
        let mut result_node = Node::default();

        for _ in 0..10 {
            let initial = Goban::new(player, enemy);
            algo.update_initial_state(initial, next_move, result_node.get_player_captures(), result_node.get_opponent_captures());
            let next_move_opt = algo.get_next_move();
            if next_move_opt.is_none() { break; }
            result_node = next_move_opt.unwrap();
            next_move = result_node.get_item().get_player() ^ initial.get_player();
            println!("Here is the next move to play for player:\n{}", next_move);
            player |= next_move;
            println!("Player's BitBoard:\n{}", player);
            let initial = Goban::new(enemy, player);
            algo.update_initial_state(initial, next_move, result_node.get_opponent_captures(), result_node.get_player_captures());
            let next_move_opt = algo.get_next_move();
            if next_move_opt.is_none() { break; }
            result_node = next_move_opt.unwrap();
            next_move = result_node.get_item().get_player() ^ initial.get_player();
            println!("Here is the next move to play for enemy:\n{}", next_move);
            enemy |= next_move;
            println!("Enemy's BitBoard:\n{}", enemy);
        }
        todo!();
    }
}