use crate::algorithm::Algorithm;
use crate::goban::fscore::Fscore;
use crate::node::{Node, Branches};
use crate::bitboard::pattern::extract_captured_by_move;
use crate::goban::Goban;


impl Algorithm {

    pub fn negamax(&self, node: &mut Node, depth: u32, mut alpha: Fscore, mut beta: Fscore) -> Node {
        if depth == 0 || self.is_game_over(node) {
            self.compute_and_set_fscore(node, depth + 1);
            return node.clone();
        }
        let mut candidate = node.clone();
        let mut fscore = Fscore::Value(isize::MIN);
        node.add_many_branches(self.node_generator_negamax(&node));
        let children : Option<&Branches> = node.get_branches();
        if let Some(children) = children {
            for child in children {
                let grandchild = self.negamax(&mut child.borrow_mut(), depth - 1, alpha, beta);
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
        candidate
    }

    //
    // fn sort_by_estimate(nodes: &mut Vec<Node>) {
    //     for (i, node) in nodes.iter().enumerate() {
    //         println!("node[{:?}] estimation is : {:?}", i, node.get_item().get_estimation());
    //     }
    // }

    // TODO: Could be more efficient to calculate the score for each new node
    // and then sort the resulting Vec<Node> according this score.
    fn node_generator_negamax(&self, parent: &Node) -> Vec<Node> {
        let parent_goban = parent.get_item();
        let parent_player = parent_goban.get_player();
        let parent_enemy = parent_goban.get_enemy();
        let parent_player_captures = parent.get_player_captures();
        let parent_enemy_captures = parent.get_opponent_captures();

        // TODO: Investigate this call and its return value (especially for open 2).
        self.get_potential_moves(parent)
            .enumerate()
            .iter()
            .map(|b| {
                let mut player_captures = parent_player_captures;
                let mut enemy_captures = parent_enemy_captures;
                let (player, enemy, is_players_move) =
                    {
                        let player_with_move = parent_player | b;
                        let captured_by_player = extract_captured_by_move(player_with_move, *parent_enemy, *b, &self.patterns);
                        if captured_by_player.is_any() {
                            player_captures += (captured_by_player.count_ones() / 2) as u8;
                            (player_with_move, parent_enemy ^ &captured_by_player, true)
                        } else {
                            (player_with_move, *parent_enemy, true)
                        }
                    };
                Node::new(Goban::new_with_estimation(player, enemy), parent.get_depth() + 1, *b, is_players_move, player_captures, enemy_captures)
            })
            .collect()
    }
}