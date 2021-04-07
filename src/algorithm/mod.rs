use crate::bitboard::{direction::Direction, pattern::*};
use crate::goban::fscore::Fscore;

use super::{bitboard::BitBoard, goban::Goban, node::Node};

#[cfg(test)]
mod tests;
mod minimax;
mod transposition_table;
mod negamax;

static PATTERNS: [((u8, u8, bool), isize, isize); 12] = [
    ((0b11111000, 5, true), 10000000000isize, 100000000isize),
    ((0b01111000, 6, true), 9999999isize, 99999999isize),
    ((0b01111000, 5, false), 1000isize, 99999999isize),
    ((0b10111000, 5, false), 50isize, 99999999isize),
    ((0b11011000, 5, true), 50isize, 99999999isize),
    ((0b01110000, 5, true), 50isize, 500isize),
    ((0b01110000, 5, false), 50isize, 500isize),
    ((0b00111000, 6, false), 3000isize, 999999isize),
    ((0b11100000, 5, false), 50isize, 1500isize),
    ((0b01011000, 5, false), 50isize, 500isize),
    ((0b01100000, 4, true), 500isize, 2000isize),
    ((0b01010000, 5, true), 25isize, 1000isize)
];

pub enum Algorithms {
    Negamax,
    Minimax
}

// Should patterns even be in Algorithm ?
#[derive(Default)]
pub struct Algorithm {
    initial: Node,
    patterns: NewPattern,
    player_captures: u8,
    opponent_captures: u8
}

impl Algorithm {
    /// Initialize a new default Algorithm object.
    pub fn new() -> Self {
        Algorithm::default()
    }

    pub fn get_initial(&self) -> &Node {
        &self.initial
    }

    /// Set the initial Node to a new state using the provided Goban.
    pub fn update_initial_state(&mut self, initial_state: Goban, last_move: BitBoard, player_captures: u8, opponent_captures:u8) {
        let new_initial_node = Node::new(initial_state, crate::DEPTH as usize, last_move, false, player_captures, opponent_captures);
        self.initial = new_initial_node;
    }

    // FIXME: SHOULDN'T BE PUBLIC
    pub fn compute_and_set_fscore(&self, node: &mut Node, depth: u32) -> Fscore {
        // If player is threatened in the initial Node then we give more weight to the defense
        // in order to prioritize the defense over the attack.
        // We do the opposite if there is no immediate threats in inital Node for player.
        let defense_weight = if self.initial.is_player_threatened() {3f64} else {1.5};
        // let defense_weight = 1.5f64;
        let player_score = self.compute_score(node, depth, false);
        let enemy_score = self.compute_score(node, depth, true);
        let global_score = match (player_score, enemy_score) {
            (Fscore::Win, _) => Fscore::Win,
            (_, Fscore::Win) => Fscore::Value(isize::MIN),
            (Fscore::Uninitialized, Fscore::Value(score)) => Fscore::Value(-score),
            (Fscore::Value(player_value), Fscore::Value(enemy_value)) => Fscore::Value(player_value - (enemy_value as f64 * defense_weight).round() as isize),
            // (Fscore::Value(player_value), Fscore::Value(enemy_value)) => Fscore::Value(player_value - enemy_value),
            (Fscore::Value(player_value), _) => Fscore::Value(player_value),
            (Fscore::Uninitialized, Fscore::Uninitialized) => Fscore::Uninitialized
        };
        node.set_item_fscore(global_score);

        global_score
    }



    // TODO: Could be more efficient to calculate the score for each new node
    // and then sort the resulting Vec<Node> according this score.
    pub(crate) fn node_generator(&self, parent: &Node, maximazing: bool) -> Vec<Node> {
        let parent_goban = parent.get_item();
        let parent_player = parent_goban.get_player();
        let parent_enemy = parent_goban.get_enemy();
        let parent_player_captures = parent.get_player_captures();
        let parent_enemy_captures = parent.get_opponent_captures();

        let mut ret : Vec<Node> = self.get_potential_moves(parent)
            .enumerate()
            .iter()
            .map(|b| {
                let mut player_captures = parent_player_captures;
                let mut enemy_captures = parent_enemy_captures;
                let (player, enemy, is_players_move) =
                    if maximazing {
                        let player_with_move = parent_player | b;
                        let captured_by_player = extract_captured_by_move(player_with_move, *parent_enemy, *b, &self.patterns);
                        if captured_by_player.is_any() {
                            player_captures += (captured_by_player.count_ones() / 2) as u8;
                            (player_with_move, parent_enemy ^ &captured_by_player, true)
                        } else {
                            (player_with_move, *parent_enemy, true)
                        }
                    } else {
                        let enemy_with_move = parent_enemy | b;
                        let captured_by_enemy = extract_captured_by_move(enemy_with_move, *parent_player, *b, &self.patterns);
                        if captured_by_enemy.is_any() {
                            enemy_captures += (captured_by_enemy.count_ones() / 2) as u8;
                            (parent_player ^ &captured_by_enemy, enemy_with_move, false)
                        } else {
                            (*parent_player, enemy_with_move, false)
                        }
                    };
                Node::new(Goban::new(player, enemy), parent.get_depth() + 1, *b, is_players_move, player_captures, enemy_captures)
            })
            .collect();
        sort_by_estimate(&mut ret);
        ret
    }

    // TODO: Missing tests
    // FIXME: This method has never been tested.
    fn compute_score(&self, node: &Node, depth: u32, player_is_enemy: bool) -> Fscore {
        let goban = node.get_item();
        let (player, enemy, player_captures, enemy_captures) = if player_is_enemy {
            (goban.get_enemy(), goban.get_player(), node.get_opponent_captures(), node.get_player_captures())
        } else {
            (goban.get_player(), goban.get_enemy(), node.get_player_captures(), node.get_opponent_captures())
        };
        let mut result = 0isize;

        if player_captures >= 5 {
            return Fscore::Value(50000000 * depth as isize);
        }
        if extract_five_aligned(player ^ &extract_captures(*enemy, *player, &self.patterns))
            .is_any()
            && extract_winning_move_capture(
                *enemy,
                *player,
                node.get_opponent_captures(),
                &self.patterns
            ).is_empty()
        {
            return Fscore::Value(50000000 * depth as isize);
        }
        let three_cross_four = extract_missing_bit_cross_three_with_four(*player, *enemy);
        if three_cross_four.is_any() {
            result += three_cross_four.count_ones() as isize * if node.is_players_last_move() { 500 } else { 1000 };
            // result += three_cross_four.count_ones() as isize * 100;
        }
        let four_cross_four = extract_missing_bit_cross_four_with_four(*player, *enemy);
        if four_cross_four.is_any() {
            result += four_cross_four.count_ones() as isize * if node.is_players_last_move() { 750 } else { 1200 };
            // result += four_cross_four.count_ones() as isize * 200;
        }
        // TODO: Let this be a global static
        // let patterns: [((u8, u8, bool), isize, isize); 12] = [
        //     (self.patterns[PatternName::OpenThree], 50isize, 500isize),
        //     (self.patterns[PatternName::CloseThree], 50isize, 1500isize),
        //     (self.patterns[PatternName::OpenSplitThreeLeft], 50isize, 500isize),
        //     (self.patterns[PatternName::OpenSplitThreeRight], 50isize, 500isize),
        //     (self.patterns[PatternName::OpenFour], 9999999isize, 99999999isize),
        //     (self.patterns[PatternName::CloseFour], 1000isize, 99999999isize),
        //     (self.patterns[PatternName::SplitFourRight], 50isize, 99999999isize),
        //     (self.patterns[PatternName::SplitFourLeft], 50isize, 99999999isize),
        //     (self.patterns[PatternName::SplitFourMiddle], 50isize, 99999999isize),
        //     (self.patterns[PatternName::Five], 10000000000isize, 100000000isize),
        //     ((0b01100000, 4, true), 500isize, 2000isize),
        //     ((0b01010000, 5, true), 25isize, 1000isize)
        // ];
        // Got score values from https://playgomoku.online/gomoku-offline
        // let patterns: [((u8, u8, bool), isize, isize); 12] = [
        //     ((0b11111000, 5, true), 10000000000isize, 100000000isize),
        //     ((0b01111000, 6, true), 9999999isize, 99999999isize),
        //     ((0b01111000, 5, false), 1000isize, 99999999isize),
        //     ((0b10111000, 5, false), 50isize, 99999999isize),
        //     ((0b11011000, 5, true), 50isize, 99999999isize),
        //     ((0b01110000, 5, true), 50isize, 500isize),
        //     ((0b01110000, 5, false), 50isize, 500isize),
        //     ((0b00111000, 6, false), 3000isize, 999999isize),
        //     ((0b11100000, 5, false), 50isize, 1500isize),
        //     ((0b01011000, 5, false), 50isize, 500isize),
        //     ((0b01100000, 4, true), 500isize, 2000isize),
        //     ((0b01010000, 5, true), 25isize, 1000isize)
        // ];
        for &((pattern, pattern_size, is_sym), player_score, opponent_score) in HEURISTIC_PATTERNS.iter() {
            let score = if node.is_players_last_move() { player_score } else { opponent_score };
            let matched = match_pattern(*player, *enemy, pattern, pattern_size, is_sym);
            let matched_captures = match_pattern(
                extract_captures(*enemy, *player, &self.patterns) ^ *player,
                *enemy,
                pattern,
                pattern_size,
                is_sym
            );
            let nb_captures = if matched_captures.is_any() {
                matched_captures.count_ones() as isize
            } else {
                0
            };
            // result += ((matched.count_ones() as isize - nb_captures) * score) + (nb_captures * score);
            result += ((matched.count_ones() as isize - nb_captures) as f64 * score as f64 * 0.25f64).round() as isize + (nb_captures * score);
        }
        result += extract_capturing_moves(*player, *enemy, &self.patterns).count_ones() as isize * if node.is_players_last_move() { 3 } else { 10 };
        result += (player_captures as isize).pow(2) * 20;

        Fscore::Value(result)
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
    fn counter_five_aligned(
        &self,
        player: BitBoard,
        opponent: BitBoard,
        player_captures: u8) -> BitBoard {
        let result = extract_five_align_breaking_moves(player, opponent, &self.patterns);

        if result.is_empty() {
            extract_winning_move_capture(player, opponent, player_captures, &self.patterns)
        } else {
            result
        }
    }

    fn is_game_over(&self, current: &Node) -> bool {
        let goban = current.get_item();
        let (player, opponent, player_captures, opponent_captures) = if current.is_players_last_move() {
            (goban.get_player(), goban.get_enemy(), current.get_player_captures(), current.get_opponent_captures())
        } else {
            (goban.get_enemy(), goban.get_player(), current.get_opponent_captures(), current.get_player_captures())
        };
        let last_move = current.get_last_move();

        // Current player wins by capture
        if player_captures >= 5 {
            return true;
        }
        // Current opponent wins by unbroken 5 alignment
        // TODO: We probably want to extract the breaking moves here instead of the 5 alignment.
        if opponent.contains_five_aligned() && (extract_five_aligned(*opponent) & last_move).is_empty() {
            return true;
        }
        if !player.contains_five_aligned() {
            // Game is over because there is no more empty cells to play
            return goban.get_board().is_full()
        }
        // Player wins by unbroken 5 alignment
        if (extract_five_aligned(*player) & last_move).is_empty() {
            return true;
        }
        // Opponent wins by unbroken 5 alignment
        if opponent.contains_five_aligned() {
            return true;
        }
        // Opponent still can break player's 5 alignment
        if extract_five_align_breaking_moves(*opponent, *player, &self.patterns).is_any() {
            return false;
        }
        // Opponent still can win by capture
        if extract_winning_move_capture(*opponent, *player, opponent_captures, &self.patterns).is_any() {
            return false;
        }
        // Player wins by unbreakable alignment
        true
    }

    pub fn compute_initial_threats_for_player(&mut self) {
        self.initial.compute_immediate_threats_for_player(&self.patterns);
    }

    // FIXME: Shouldn't be public (made it pub for debug)
    pub fn get_potential_moves(&self, parent: &Node) -> BitBoard {
        let goban = parent.get_item();
        // If the Node parent is representing a move for player then it means we are generating moves for opponent
        let (current_player, opponent, player_captures, opponent_captures) = if parent.is_players_last_move() {
            (*goban.get_enemy(), *goban.get_player(), parent.get_opponent_captures(), parent.get_player_captures())
        } else {
            (*goban.get_player(), *goban.get_enemy(), parent.get_player_captures(), parent.get_opponent_captures())
        };
        // let player_captures = parent.get_player_captures();
        // let opponent_captures = parent.get_opponent_captures();
        let open_cells = !(current_player | opponent);
        let illegal_moves_complement = !extract_illegal_moves(current_player, opponent, &self.patterns);
        let legal_open_cells = open_cells & illegal_moves_complement;

        if current_player.is_empty() {
            return open_cells & Self::get_first_move(current_player, opponent);
        }

        if opponent.contains_five_aligned() {
            let result = self.counter_five_aligned(current_player, opponent, player_captures) & legal_open_cells;
            if result.is_any() {
                return result;
            }
        }

        let result = extract_winning_moves_from_player(
            current_player,
            opponent,
            player_captures,
            opponent_captures,
            &self.patterns
        ) & illegal_moves_complement;
        if result.is_any() {
            return result;
        }

        let result = extract_winning_moves_from_player(
            opponent,
            current_player,
            opponent_captures,
            player_captures,
            &self.patterns
        ) & illegal_moves_complement;
        if result.is_any() {
            let (pattern, pattern_size, is_sym) = self.patterns[PatternName::Five];
            return result | extract_missing_bit(current_player, opponent, pattern, pattern_size, is_sym);
        }

        // Get the moves that threat `player` to be able to play the move before the opponent does.
        let mut result = extract_threatening_moves_from_player(
            current_player,
            opponent,
            opponent_captures,
            &self.patterns
        );

        // Get the moves that threat `opponent` because those are good move to play.
        result |= extract_threatening_moves_from_player(
            opponent,
            current_player,
            player_captures,
            &self.patterns
        );
        if !is_threatened || opponent_captures >= 6 {
            result |= extract_capturing_moves(opponent, current_player, &self.patterns);
        }
        result |= extract_missing_bit(
            current_player,
            opponent,
            GET_MOVES_PATTERNS[0].0,
            GET_MOVES_PATTERNS[0].1,
            GET_MOVES_PATTERNS[0].2
        );
        result |= extract_missing_bit(
            current_player,
            opponent,
            GET_MOVES_PATTERNS[1].0,
            GET_MOVES_PATTERNS[1].1,
            GET_MOVES_PATTERNS[1].2
        );
        result |= extract_capturing_moves(current_player, opponent, &self.patterns);
        result &= legal_open_cells;

        if result.count_ones() > 4 {
            return result;
        }

        result |= extract_threatening_moves_from_opponent(
            current_player,
            opponent,
            GET_MOVES_PATTERNS[2].0,
            GET_MOVES_PATTERNS[2].1,
            GET_MOVES_PATTERNS[2].2
        );
        result |= extract_missing_bit(
            current_player,
            opponent,
            GET_MOVES_PATTERNS[3].0,
            GET_MOVES_PATTERNS[3].1,
            GET_MOVES_PATTERNS[3].2
        );
        result |= extract_missing_bit(
            current_player,
            opponent,
            GET_MOVES_PATTERNS[4].0,
            GET_MOVES_PATTERNS[4].1,
            GET_MOVES_PATTERNS[4].2
        );
        result &= legal_open_cells;

        if result.count_ones() > 4 {
            return result;
        }

        result |= extract_missing_bit(
            current_player,
            opponent,
            GET_MOVES_PATTERNS[5].0,
            GET_MOVES_PATTERNS[5].1,
            GET_MOVES_PATTERNS[5].2
        );
        result &= legal_open_cells;

        if result.count_ones() > 2 || is_threatened {
            return result;
        }

        (result | (current_player + Direction::All)) & legal_open_cells
    }

    // TODO: We maybe can do better here, self probably doesn't need to be mutable.
    // Maybe we should pass the initial Node directly without passing by the initial property of Algorithm?
    /// This method is likely to change in a near future because I'm not sure what to return.
    /// For now, it returns a BitBoard that contains the next move to play.
    pub fn get_next_move(&mut self, depth: u32, algo: Algorithms) -> Option<Node> {
        self.compute_initial_threats_for_player();
        let mut initial = self.initial.clone();
        let next_state = match algo {
            Algorithms::Negamax => self.negamax(&mut initial, depth, Fscore::MIN, Fscore::MAX),
            Algorithms::Minimax => self.minimax(&mut initial, depth, Fscore::MIN, Fscore::MAX, true)
        };
        if next_state == self.initial {
            None
        } else {
            Some(next_state)
            // Some(next_state.get_item().get_player() ^ self.initial.get_item().get_player())
        }
    }
}

fn sort_by_estimate(nodes: &mut Vec<Node>) {
    nodes.sort_unstable_by(|a, b| b.get_item().get_fscore().partial_cmp(&a.get_item().get_fscore()).unwrap());
}
