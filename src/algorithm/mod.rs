use crate::bitboard::{direction::Direction, pattern::*};
use crate::goban::fscore::Fscore;

use super::{bitboard::BitBoard, goban::Goban, node::Node};

#[cfg(test)]
mod tests;
mod minimax;
mod transposition_table;
mod negamax;

#[derive(Default)]
pub struct Algorithm {
    initial: Node,
    patterns: NewPattern,
}

impl Algorithm {
    /// Initialize a new default Algorithm object.
    pub fn new() -> Self {
        Algorithm::default()
    }

    /// Set the initial Node to a new state using the provided Goban.
    pub fn update_initial_state(&mut self, initial_state: Goban, last_move: BitBoard, player_captures: u8, opponent_captures:u8) {
        let new_initial_node = Node::new(initial_state, crate::DEPTH, last_move, false, player_captures, opponent_captures);
        self.initial = new_initial_node;
    }

    fn compute_and_set_fscore(&self, node: &mut Node, depth: u32) -> Fscore {
        // If player is threatened in the initial Node then we give more weight to the defense
        // in order to prioritize the defense over the attack.
        // We do the opposite if there is no immediate threats in initial Node for player.
        // let defense_weight = if self.initial.is_player_threatened() {3f64} else {1.5};
        let defense_weight = 1.5f64;
        let player_score = self.compute_score(node, depth, false);
        let enemy_score = self.compute_score(node, depth, true);
        let global_score = match (player_score, enemy_score) {
            (Fscore::Win, _) => Fscore::Win,
            (_, Fscore::Win) => Fscore::Value(isize::MIN),
            (Fscore::Uninitialized, Fscore::Value(score)) => Fscore::Value(-score),
            (Fscore::Value(player_value), Fscore::Value(enemy_value)) => Fscore::Value(player_value - (enemy_value as f64 * defense_weight).round() as isize),
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
                Node::new(Goban::new(player, enemy), parent.get_depth() - 1, *b, is_players_move, player_captures, enemy_captures)
            })
            .collect();
        sort_by_estimate(&mut ret, maximazing);
        ret
    }

    // FIXME: This method has never been tested.
    fn compute_score(&self, node: &Node, depth: u32, player_is_enemy: bool) -> Fscore {
        let goban = node.get_item();
        let (player, enemy, player_captures) = if player_is_enemy {
            (goban.get_enemy(), goban.get_player(), node.get_opponent_captures())
        } else {
            (goban.get_player(), goban.get_enemy(), node.get_player_captures())
        };
        let mut result = 0isize;
        let is_attack_score = (player_is_enemy && !node.is_players_last_move()) || (!player_is_enemy && node.is_players_last_move());

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
            result += three_cross_four.count_ones() as isize * if is_attack_score { 500 } else { 1000 };
        }
        let four_cross_four = extract_missing_bit_cross_four_with_four(*player, *enemy);
        if four_cross_four.is_any() {
            result += four_cross_four.count_ones() as isize * if is_attack_score { 750 } else { 1200 };
        }
        for &((pattern, pattern_size, is_sym), player_score, opponent_score) in HEURISTIC_PATTERNS.iter() {
            let score = if is_attack_score { player_score } else { opponent_score };
            // let score = if node.is_players_last_move() { player_score } else { opponent_score };
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
            result += ((matched.count_ones() as isize - nb_captures) as f64 * score as f64 * 0.25f64).round() as isize + (nb_captures * score);
        }
        result += extract_capturing_moves(*player, *enemy, &self.patterns).count_ones() as isize * if is_attack_score { 3 } else { 10 };
        // result += extract_capturing_moves(*player, *enemy, &self.patterns).count_ones() as isize * if node.is_players_last_move() { 3 } else { 10 };
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

    fn get_potential_moves(&self, parent: &Node) -> BitBoard {
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
            // Those are moves that perform a capture on the opponent's stones.
            // If one of those moves breaks the threatening alignment we want to consider that move.
            // let capturing_moves_by_player = extract_capturing_moves(current_player, opponent, &self.patterns);
            let (pattern, pattern_size, is_sym) = self.patterns[PatternName::Five];
            let moves_to_complete_five = extract_missing_bit(opponent, current_player, pattern, pattern_size, is_sym);
            let breaking_moves = extract_five_align_breaking_moves(current_player, moves_to_complete_five | opponent, &self.patterns);
            // let potential_five = extract_five_aligned(extract_missing_bit(current_player, opponent, pattern, pattern_size, is_sym) | current_player);
            // let dilated_potential_five = potential_five + Direction::All;
            // let potential_five_with_edges = dilated_potential_five & current_player;
            return result | moves_to_complete_five | breaking_moves;
        }

        // Get the moves that threat `player` to be able to play the move before the opponent does.
        let mut result = extract_threatening_moves_from_player(
            current_player,
            opponent,
            opponent_captures,
            &self.patterns
        );
        let is_threatened = result.is_any();

        // Get the moves that threat `opponent` because those are good move to play.
        result |= extract_threatening_moves_from_player(
            opponent,
            current_player,
            player_captures,
            &self.patterns
        );
        if !is_threatened || opponent_captures >= 3 {
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

    // Maybe we should pass the initial Node directly without passing by the initial property of Algorithm?
    /// For now, it returns a Node that contains the next move to play.
    pub fn get_next_move(&mut self, depth: u32) -> Node {
        self.compute_initial_threats_for_player();
        let mut initial = self.initial.clone();
        let next_state = self.minimax(&mut initial, depth, Fscore::MIN, Fscore::MAX, true);

        next_state
    }
}

fn sort_by_estimate(nodes: &mut Vec<Node>, maximize: bool) {
    if maximize {
        nodes.sort_unstable_by(|a, b| b.get_item().get_fscore().partial_cmp(&a.get_item().get_fscore()).unwrap());
    } else {
        nodes.sort_unstable_by(|a, b| a.get_item().get_fscore().partial_cmp(&b.get_item().get_fscore()).unwrap());
    }
}