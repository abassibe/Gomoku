use crate::bitboard::{direction::Direction, pattern::*};
use crate::goban::fscore::Fscore;

use super::{bitboard::BitBoard, goban::Goban, node::Node};

#[cfg(test)]
mod tests;
mod minimax;

#[derive(Default)]
pub struct Algorithm {
    initial: Node,
    patterns: NewPattern,
    computer_captures: u8,
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
    pub fn update_initial_state(&mut self, initial_state: Goban, last_move: BitBoard, computer_captures: u8, opponent_captures:u8) {
        let new_initial_node = Node::new(initial_state, 0, last_move, false, computer_captures, opponent_captures);
        self.initial = new_initial_node;
    }

    // FIXME: SHOULDN'T BE PUBLIC
    pub fn compute_and_set_fscore(&self, node: &mut Node, depth: u32) -> Fscore {
        // If computer is threatened in the initial Node then we give more weight to the defense
        // in order to prioritize the defense over the attack.
        // We do the opposite if there is no immediate threats in initial Node for computer.
        let defense_weight = 1.5f64;
        let computer_score = self.compute_score(node, depth, false);
        let human_score = self.compute_score(node, depth, true);
        let global_score = match (computer_score, human_score) {
            (Fscore::Win, _) => Fscore::Win,
            (_, Fscore::Win) => Fscore::Value(isize::MIN),
            (Fscore::Uninitialized, Fscore::Value(score)) => Fscore::Value(-score),
            (Fscore::Value(computer_value), Fscore::Value(human_value)) => Fscore::Value(computer_value - (human_value as f64 * defense_weight).round() as isize),
            // (Fscore::Value(computer_value), Fscore::Value(human_value)) => Fscore::Value(computer_value - human_value),
            (Fscore::Value(computer_value), _) => Fscore::Value(computer_value),
            (Fscore::Uninitialized, Fscore::Uninitialized) => Fscore::Uninitialized
        };
        node.set_item_fscore(global_score);

        global_score
    }

    // TODO: Missing tests
    // FIXME: This method has never been tested.
    fn compute_score(&self, node: &Node, depth: u32, current_player_is_human: bool) -> Fscore {
        let goban = node.get_item();
        let (computer, human, computer_captures, human_captures) = if current_player_is_human {
            (goban.get_human(), goban.get_computer(), node.get_opponent_captures(), node.get_computer_captures())
        } else {
            (goban.get_computer(), goban.get_human(), node.get_computer_captures(), node.get_opponent_captures())
        };
        let mut result = 0isize;

        if computer_captures >= 5 {
            return Fscore::Value(10000000000 * depth as isize);
        }
        if extract_five_aligned(computer ^ &extract_captures(*human, *computer, &self.patterns))
            .is_any()
            && extract_winning_move_capture(
                *human,
                *computer,
                node.get_opponent_captures(),
                &self.patterns
            ).is_empty()
        {
            return Fscore::Value(10000000000 * depth as isize);
        }
        let three_cross_four = extract_missing_bit_cross_three_with_four(*computer, *human);
        if three_cross_four.is_any() {
            result += three_cross_four.count_ones() as isize * if node.is_computers_last_move() { 500 } else { 1000 };
        }
        let four_cross_four = extract_missing_bit_cross_four_with_four(*computer, *human);
        if four_cross_four.is_any() {
            result += four_cross_four.count_ones() as isize * if node.is_computers_last_move() { 750 } else { 1200 };
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
        let patterns: [((u8, u8, bool), isize, isize); 12] = [
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
        for &((pattern, pattern_size, is_sym), computer_score, opponent_score) in patterns.iter() {
            let score = if node.is_computers_last_move() { computer_score } else { opponent_score };
            let matched = match_pattern(*computer, *human, pattern, pattern_size, is_sym);
            let matched_captures = match_pattern(
                extract_captures(*human, *computer, &self.patterns) ^ *computer,
                *human,
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
        result += extract_capturing_moves(*computer, *human, &self.patterns).count_ones() as isize * if node.is_computers_last_move() { 3 } else { 10 };
        result += (computer_captures as isize).pow(2) * 20;

        Fscore::Value(result)
    }

    // TODO: Could be more efficient to calculate the score for each new node
    // and then sort the resulting Vec<Node> according this score.
    fn node_generator(&self, parent: &Node, maximazing: bool) -> Vec<Node> {
        let parent_goban = parent.get_item();
        let parent_computer = parent_goban.get_computer();
        let parent_human = parent_goban.get_human();
        let parent_computer_captures = parent.get_computer_captures();
        let parent_human_captures = parent.get_opponent_captures();

        // TODO: Investigate this call and its return value (especially for open 2).
        self.get_potential_moves(parent)
            .enumerate()
            .iter()
            .map(|b| {
                let mut computer_captures = parent_computer_captures;
                let mut human_captures = parent_human_captures;
                let (computer, human, is_computers_move) =
                    if maximazing {
                        let computer_with_move = parent_computer | b;
                        let captured_by_computer = extract_captured_by_move(computer_with_move, *parent_human, *b, &self.patterns);
                        if captured_by_computer.is_any() {
                            computer_captures += (captured_by_computer.count_ones() / 2) as u8;
                            (computer_with_move, parent_human ^ &captured_by_computer, true)
                        } else {
                            (computer_with_move, *parent_human, true)
                        }
                    } else {
                        let human_with_move = parent_human | b;
                        let captured_by_human = extract_captured_by_move(human_with_move, *parent_computer, *b, &self.patterns);
                        if captured_by_human.is_any() {
                            human_captures += (captured_by_human.count_ones() / 2) as u8;
                            (parent_computer ^ &captured_by_human, human_with_move, false)
                        } else {
                            (*parent_computer, human_with_move, false)
                        }
                    };
                Node::new(Goban::new(computer, human), parent.get_depth() + 1, *b, is_computers_move, computer_captures, human_captures)
            })
            .collect()
    }

    #[inline]
    fn get_first_move(computer: BitBoard, opponent: BitBoard) -> BitBoard {
        match (computer.is_empty(), opponent.is_empty()) {
            (true, false) => (opponent + Direction::All) & !computer,
            (true, true) => BitBoard::CENTER_BIT_SET,
            (false, _) => BitBoard::empty()
        }
    }

    #[inline]
    fn counter_five_aligned(
        &self,
        computer: BitBoard,
        opponent: BitBoard,
        computer_captures: u8) -> BitBoard {
        let result = extract_five_align_breaking_moves(computer, opponent, &self.patterns);

        if result.is_empty() {
            extract_winning_move_capture(computer, opponent, computer_captures, &self.patterns)
        } else {
            result
        }
    }

    fn is_game_over(&self, current: &Node) -> bool {
        let goban = current.get_item();
        let (computer, opponent, computer_captures, opponent_captures) = if current.is_computers_last_move() {
            (goban.get_computer(), goban.get_human(), current.get_computer_captures(), current.get_opponent_captures())
        } else {
            (goban.get_human(), goban.get_computer(), current.get_opponent_captures(), current.get_computer_captures())
        };
        let last_move = current.get_last_move();

        // Current computer wins by capture
        if computer_captures >= 5 {
            return true;
        }
        // Current opponent wins by unbroken 5 alignment
        // TODO: We probably want to extract the breaking moves here instead of the 5 alignment.
        if opponent.contains_five_aligned() && (extract_five_aligned(*opponent) & last_move).is_empty() {
            return true;
        }
        if !computer.contains_five_aligned() {
            // Game is over because there is no more empty cells to play
            if goban.get_board().is_full() {
                return true;
            }
            // The game is not over yet
            else {
                return false;
            }
        }
        // Computer wins by unbroken 5 alignment
        if (extract_five_aligned(*computer) & last_move).is_empty() {
            return true;
        }
        // Opponent wins by unbroken 5 alignment
        if opponent.contains_five_aligned() {
            return true;
        }
        // Opponent still can break computer's 5 alignment
        if extract_five_align_breaking_moves(*opponent, *computer, &self.patterns).is_any() {
            return false;
        }
        // Opponent still can win by capture
        if extract_winning_move_capture(*opponent, *computer, opponent_captures, &self.patterns).is_any() {
            return false;
        }
        // Computer wins by unbreakable alignment
        return true;
    }

    pub fn compute_initial_threats_for_computer(&mut self) {
        self.initial.compute_immediate_threats_for_computer(&self.patterns);
    }

    // FIXME: Shouldn't be public (made it pub for debug)
    pub fn get_potential_moves(&self, parent: &Node) -> BitBoard {
        let goban = parent.get_item();
        // If the Node parent is representing a move for computer then it means we are generating moves for opponent
        let (current_computer, opponent) = if parent.is_computers_last_move() {
            (*goban.get_human(), *goban.get_computer())
        } else {
            (*goban.get_computer(), *goban.get_human())
        };
        let computer_captures = parent.get_computer_captures();
        let opponent_captures = parent.get_opponent_captures();
        let open_cells = !(current_computer | opponent);
        let illegal_moves_complement = !extract_illegal_moves(current_computer, opponent, &self.patterns);
        let legal_open_cells = open_cells & illegal_moves_complement;

        if current_computer.is_empty() {
            return open_cells & Self::get_first_move(current_computer, opponent);
        }

        if opponent.contains_five_aligned() {
            let result = self.counter_five_aligned(current_computer, opponent, computer_captures) & legal_open_cells;
            if result.is_any() {
                return result;
            }
        }

        let result = extract_winning_moves_from_computer(
            current_computer,
            opponent,
            computer_captures,
            opponent_captures,
            &self.patterns
        ) & illegal_moves_complement;
        if result.is_any() {
            return result;
        }

        let result = extract_winning_moves_from_computer(
            opponent,
            current_computer,
            opponent_captures,
            computer_captures,
            &self.patterns
        ) & illegal_moves_complement;
        if result.is_any() {
            let (pattern, pattern_size, is_sym) = self.patterns[PatternName::Five];
            return result | extract_missing_bit(current_computer, opponent, pattern, pattern_size, is_sym);
        }

        // Get the moves that theat `computer` to be able to play the move before the opponent does.
        let mut result = extract_threatening_moves_from_computer(
            current_computer,
            opponent,
            opponent_captures,
            &self.patterns
        );

        // Get the moves that theat `opponent` because those are good move to play.
        result |= extract_threatening_moves_from_computer(
            opponent,
            current_computer,
            computer_captures,
            &self.patterns
        );
        result |= extract_capturing_moves(opponent, current_computer, &self.patterns);
        result |= extract_missing_bit(
            current_computer,
            opponent,
            GET_MOVES_PATTERNS[0].0,
            GET_MOVES_PATTERNS[0].1,
            GET_MOVES_PATTERNS[0].2
        );
        result |= extract_missing_bit(
            current_computer,
            opponent,
            GET_MOVES_PATTERNS[1].0,
            GET_MOVES_PATTERNS[1].1,
            GET_MOVES_PATTERNS[1].2
        );
        result |= extract_capturing_moves(current_computer, opponent, &self.patterns);
        result &= legal_open_cells;

        if result.count_ones() > 4 {
            return result;
        }

        result |= extract_threatening_moves_from_opponent(
            current_computer,
            opponent,
            GET_MOVES_PATTERNS[2].0,
            GET_MOVES_PATTERNS[2].1,
            GET_MOVES_PATTERNS[2].2
        );
        result |= extract_missing_bit(
            current_computer,
            opponent,
            GET_MOVES_PATTERNS[3].0,
            GET_MOVES_PATTERNS[3].1,
            GET_MOVES_PATTERNS[3].2
        );
        result |= extract_missing_bit(
            current_computer,
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
            current_computer,
            opponent,
            GET_MOVES_PATTERNS[5].0,
            GET_MOVES_PATTERNS[5].1,
            GET_MOVES_PATTERNS[5].2
        );
        result &= legal_open_cells;

        if result.count_ones() > 2 {
            return result;
        }

        (result | (current_computer + Direction::All)) & legal_open_cells
    }

    // TODO: We maybe can do better here, self probably doesn't need to be mutable.
    // Maybe we should pass the initial Node directly without passing by the initial property of Algorithm?
    /// This method is likely to change in a near future because I'm not sure what to return.
    /// For now, it returns a BitBoard that contains the next move to play.
    pub fn get_next_move(&mut self, depth: u32) -> Option<Node> {
        self.compute_initial_threats_for_computer();
        let mut initial = self.initial.clone();
        let next_state = self.minimax(&mut initial, depth, Fscore::MIN, Fscore::MAX, true);
        if next_state == self.initial {
            None
        } else {
            Some(next_state)
            // Some(next_state.get_item().get_computer() ^ self.initial.get_item().get_computer())
        }
    }
}
