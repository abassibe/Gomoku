#[cfg(test)]
mod tests;

use crate::bitboard::{direction::Direction, new_pattern::*};

use super::{
    bitboard::BitBoard,
    goban::{Fscore, Goban},
    node::Node,
};

#[derive(Default)]
pub struct Algorithm {
    initial: Node,
    patterns: NewPattern,
    player_captures: u8,
    opponent_captures: u8,
}

impl Algorithm {
    /// Initialize a new default Algorithm object.
    pub fn new() -> Self {
        Algorithm::default()
    }

    /// Set the initial Node to a new state using the provided Goban.
    pub fn update_initial_state(
        &mut self,
        initial_state: Goban,
        last_move: BitBoard,
        player_captures: u8,
        opponent_captures: u8,
    ) {
        let new_initial_node = Node::new(
            initial_state,
            0,
            last_move,
            player_captures,
            opponent_captures,
        );
        self.initial = new_initial_node;
    }

    fn compute_and_set_fscore(&self, node: &mut Node) -> Fscore {
        let player_score = self.compute_score(node, false);
        let enemy_score = self.compute_score(node, true);
        let global_score = match (player_score, enemy_score) {
            (Fscore::Win, _) => Fscore::Win,
            (_, Fscore::Win) => Fscore::Value(isize::MIN),
            (Fscore::Uninitialized, Fscore::Value(score)) => Fscore::Value(-score),
            (Fscore::Value(player_value), Fscore::Value(enemy_value)) => {
                Fscore::Value(player_value - enemy_value)
            }
            (Fscore::Value(player_value), _) => Fscore::Value(player_value),
            (Fscore::Uninitialized, Fscore::Uninitialized) => Fscore::Uninitialized,
        };
        node.set_item_fscore(global_score);

        global_score
    }

    // TODO: Missing tests
    // FIXME: This method has never been tested.
    fn compute_score(&self, node: &Node, player_is_enemy: bool) -> Fscore {
        let goban = node.get_item();
        let (player, enemy, player_captures, enemy_captures) = if player_is_enemy {
            (
                goban.get_enemy(),
                goban.get_player(),
                node.get_opponent_captures(),
                node.get_player_captures(),
            )
        } else {
            (
                goban.get_player(),
                goban.get_enemy(),
                node.get_player_captures(),
                node.get_opponent_captures(),
            )
        };
        let mut result = 0isize;

        if player_captures >= 5 {
            return Fscore::Win;
        }
        if extract_five_aligned(player ^ &extract_captures(*enemy, *player, &self.patterns))
            .is_any()
            && extract_winning_move_capture(
                *enemy,
                *player,
                node.get_opponent_captures(),
                &self.patterns,
            )
            .is_empty()
        {
            return Fscore::Win;
        }
        let three_cross_four = extract_missing_bit_cross_three_with_four(*player, *enemy);
        if three_cross_four.is_any() {
            result += three_cross_four.count_ones() as isize * 100;
        }
        let four_cross_four = extract_missing_bit_cross_four_with_four(*player, *enemy);
        if four_cross_four.is_any() {
            result += four_cross_four.count_ones() as isize * 200;
        }
        let patterns: [((u8, u8, bool), isize); 10] = [
            (self.patterns[PatternName::OpenThree], 200isize),
            (self.patterns[PatternName::CloseThree], 50isize),
            (self.patterns[PatternName::OpenSplitThreeLeft], 200isize),
            (self.patterns[PatternName::OpenSplitThreeRight], 200isize),
            (self.patterns[PatternName::OpenFour], 1000isize),
            (self.patterns[PatternName::CloseFour], 500isize),
            (self.patterns[PatternName::SplitFourRight], 500isize),
            (self.patterns[PatternName::SplitFourLeft], 500isize),
            (self.patterns[PatternName::SplitFourMiddle], 500isize),
            (self.patterns[PatternName::Five], 10000isize),
        ];
        for &((pattern, pattern_size, is_sym), score) in patterns.iter() {
            let matched = match_pattern(*player, *enemy, pattern, pattern_size, is_sym);
            let matched_captures = match_pattern(
                extract_captures(*enemy, *player, &self.patterns) ^ *player,
                *enemy,
                pattern,
                pattern_size,
                is_sym,
            );
            let nb_captures = if matched_captures.is_any() {
                matched_captures.count_ones() as isize
            } else {
                0
            };
            result +=
                ((matched.count_ones() as isize - nb_captures) * score) + (nb_captures * score);
        }
        result +=
            extract_capturing_moves(*player, *enemy, &self.patterns).count_ones() as isize * 10;
        result += (player_captures as isize).pow(2) * 20;

        Fscore::Value(result)
    }

    // TODO: There is a lot of duplicated code in this function, we should refactor it.
    fn minimax(
        &self,
        node: &mut Node,
        depth: u32,
        mut alpha: Fscore,
        mut beta: Fscore,
        maximizing: bool,
    ) -> Node {
        if depth == 0 {
            // TODO: We have to pass the potential next move to compute_item_fscore, but we don't have it at this point
            // and I'm not even sure we actually need it, maybe we should remove it completely?
            // node.compute_item_fscore(&current_goban, current_goban.get_player(), depth as usize);
            self.compute_and_set_fscore(node);
            return node.clone();
        }
        let mut candidate = node.clone();
        let mut fscore = node.get_item().get_fscore();
        if fscore.is_win() {
            return candidate;
        }

        if maximizing {
            fscore = Fscore::Value(isize::MIN);
            node.add_many_branches(self.node_generator(&node, maximizing));
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild =
                        self.minimax(&mut child.borrow_mut(), depth - 1, alpha, beta, !maximizing);
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
        } else {
            fscore = Fscore::Value(isize::MAX);
            node.add_many_branches(self.node_generator(&node, maximizing));
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild =
                        self.minimax(&mut child.borrow_mut(), depth - 1, alpha, beta, !maximizing);
                    let grandchild_fscore = grandchild.get_item().get_fscore();
                    child.borrow_mut().set_item_fscore(grandchild_fscore);
                    if fscore > grandchild_fscore {
                        candidate = child.borrow().clone();
                        fscore = grandchild_fscore;
                    }
                    beta = beta.min(grandchild_fscore);
                    if beta >= alpha {
                        break;
                    }
                }
            }
        }

        candidate
    }

    // TODO: Could be more efficient to calculate the score for each new node
    // and then sort the resulting Vec<Node> according this score.
    fn node_generator(&self, parent: &Node, maximazing: bool) -> Vec<Node> {
        let parent_goban = parent.get_item();
        let parent_player = parent_goban.get_player();
        let parent_enemy = parent_goban.get_enemy();
        let parent_player_captures = parent.get_player_captures();
        let parent_enemy_captures = parent.get_opponent_captures();

        self.get_potential_moves(parent)
            .enumerate()
            .iter()
            .map(|b| {
                let mut player_captures = parent_player_captures;
                let mut enemy_captures = parent_enemy_captures;
                let (player, enemy) = if maximazing {
                    let player_with_move = parent_player | b;
                    let captured_by_player = extract_captured_by_move(
                        player_with_move,
                        *parent_enemy,
                        *b,
                        &self.patterns,
                    );
                    if captured_by_player.is_any() {
                        player_captures += (captured_by_player.count_ones() / 2) as u8;
                        (player_with_move, parent_enemy ^ &captured_by_player)
                    } else {
                        (player_with_move, *parent_enemy)
                    }
                } else {
                    let enemy_with_move = parent_enemy | b;
                    let captured_by_enemy = extract_captured_by_move(
                        enemy_with_move,
                        *parent_player,
                        *b,
                        &self.patterns,
                    );
                    if captured_by_enemy.is_any() {
                        enemy_captures += (captured_by_enemy.count_ones() / 2) as u8;
                        (parent_player ^ &captured_by_enemy, enemy_with_move)
                    } else {
                        (*parent_player, enemy_with_move)
                    }
                };
                Node::new(
                    Goban::new(player, enemy),
                    parent.get_depth() + 1,
                    *b,
                    player_captures,
                    enemy_captures,
                )
            })
            .collect()
    }

    #[inline]
    fn get_first_move(player: BitBoard, opponent: BitBoard) -> BitBoard {
        match (player.is_empty(), opponent.is_empty()) {
            (true, false) => (opponent + Direction::All) & !player,
            (true, true) => BitBoard::CENTER_BIT_SET,
            (false, _) => BitBoard::empty(),
        }
    }

    #[inline]
    fn counter_five_aligned(
        &self,
        player: BitBoard,
        opponent: BitBoard,
        player_captures: u8,
    ) -> BitBoard {
        let result = extract_five_align_breaking_moves(player, opponent, &self.patterns);

        if result.is_empty() {
            extract_winning_move_capture(player, opponent, player_captures, &self.patterns)
        } else {
            result
        }
    }

    fn get_potential_moves(&self, parent: &Node) -> BitBoard {
        let goban = parent.get_item();
        let player = *goban.get_player();
        let opponent = *goban.get_enemy();
        let player_captures = parent.get_player_captures();
        let opponent_captures = parent.get_opponent_captures();
        let open_cells = !(player | opponent);
        let illegal_moves_complement = !extract_illegal_moves(player, opponent, &self.patterns);

        if player.is_empty() {
            return open_cells & Self::get_first_move(player, opponent);
        }

        if opponent.contains_five_aligned() {
            let result = self.counter_five_aligned(player, opponent, player_captures);
            if result.is_any() {
                return result & open_cells;
            }
        }

        let result = extract_winning_moves_from_player(
            player,
            opponent,
            player_captures,
            opponent_captures,
            &self.patterns,
        );
        if result.is_any() {
            return result;
        }

        let result = extract_winning_moves_from_player(
            opponent,
            player,
            opponent_captures,
            player_captures,
            &self.patterns,
        );
        if result.is_any() {
            let (pattern, pattern_size, is_sym) = self.patterns[PatternName::Five];
            return result | extract_missing_bit(player, opponent, pattern, pattern_size, is_sym);
        }

        let mut result = extract_threatening_moves_from_player(
            player,
            opponent,
            opponent_captures,
            &self.patterns,
        );
        result |= extract_capturing_moves(opponent, player, &self.patterns);
        result |= extract_missing_bit(
            player,
            opponent,
            GET_MOVES_PATTERNS[0].0,
            GET_MOVES_PATTERNS[0].1,
            GET_MOVES_PATTERNS[0].2,
        );
        result |= extract_missing_bit(
            player,
            opponent,
            GET_MOVES_PATTERNS[1].0,
            GET_MOVES_PATTERNS[1].1,
            GET_MOVES_PATTERNS[1].2,
        );
        result |= extract_capturing_moves(player, opponent, &self.patterns);

        if result.count_ones() > 4 {
            return result & open_cells & illegal_moves_complement;
        }

        result |= extract_threatening_moves_from_opponent(
            player,
            opponent,
            GET_MOVES_PATTERNS[2].0,
            GET_MOVES_PATTERNS[2].1,
            GET_MOVES_PATTERNS[2].2,
        );
        result |= extract_missing_bit(
            player,
            opponent,
            GET_MOVES_PATTERNS[3].0,
            GET_MOVES_PATTERNS[3].1,
            GET_MOVES_PATTERNS[3].2,
        );
        result |= extract_missing_bit(
            player,
            opponent,
            GET_MOVES_PATTERNS[4].0,
            GET_MOVES_PATTERNS[4].1,
            GET_MOVES_PATTERNS[4].2,
        );

        if result.count_ones() > 4 {
            return result & open_cells & illegal_moves_complement;
        }

        result |= extract_missing_bit(
            player,
            opponent,
            GET_MOVES_PATTERNS[5].0,
            GET_MOVES_PATTERNS[5].1,
            GET_MOVES_PATTERNS[5].2,
        );

        if result.count_ones() > 2 {
            return result & open_cells & illegal_moves_complement;
        }

        (result | (player + Direction::All)) & open_cells & illegal_moves_complement
    }

    // TODO: We maybe can do better here, self probably doesn't need to be mutable.
    // Maybe we should pass the initial Node directly without passing by the initial property of Algorithm?
    /// This method is likely to change in a near future because I'm not sure what to return.
    /// For now, it returns a BitBoard that contains the next move to play.
    pub fn get_next_move(&mut self, depth: u32) -> Option<Node> {
        let mut initial = self.initial.clone();
        let next_state = self.minimax(&mut initial, depth, Fscore::MIN, Fscore::MAX, true);
        if next_state == self.initial {
            println!("{}", next_state); //to remove
            None
        } else {
            Some(next_state)
            // Some(next_state.get_item().get_player() ^ self.initial.get_item().get_player())
        }
    }
}
