use crate::algorithm::Algorithm;
use crate::goban::fscore::Fscore;
use crate::node::{Node, Branches};
use crate::goban::Goban;
use crate::bitboard::pattern::extract_captured_by_move;

// Not sure if this is a good idea, just trying it out.
impl Algorithm {
    // TODO: There is a lot of duplicated code in this function, we should refactor it.
    pub(super) fn minimax(&self, node: &mut Node, depth: u32, mut alpha: Fscore, mut beta: Fscore, maximizing: bool) -> Node {
        if depth == 0 || self.is_game_over(node) {
            // TODO: We have to pass the potential next move to compute_item_fscore, but we don't have it at this point
            // and I'm not even sure we actually need it, maybe we should remove it completely?
            // node.compute_item_fscore(&current_goban, current_goban.get_player(), depth as usize);
            self.compute_and_set_fscore(node, depth + 1);
            return node.clone();
        }
        let mut candidate = node.clone();

        if maximizing {
            let mut fscore = Fscore::Value(isize::MIN);
            node.add_many_branches(self.node_generator(&node, maximizing));
            println!("node fscore estimation : {:?}", node.get_item().get_estimation());
            let children : Option<&Branches> = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild = self.minimax(&mut child.borrow_mut(), depth - 1, alpha, beta, !maximizing);
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
            let mut fscore = Fscore::Value(isize::MAX);
            node.add_many_branches(self.node_generator(&node, maximizing));
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild = self.minimax(&mut child.borrow_mut(), depth - 1, alpha, beta, !maximizing);
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

    fn sort_by_estimate(nodes: &mut Vec<Node>) {
        for (i, node) in nodes.iter().enumerate() {
            println!("(before) node[{:?}] estimation is : {:?}", i, node.get_item().get_estimation());
        }
        nodes.sort_unstable_by(|a, b| b.get_item().get_estimation().partial_cmp(&a.get_item().get_estimation()).unwrap());
        for (i, node) in nodes.iter().enumerate() {
            println!("(after) node[{:?}] estimation is : {:?}", i, node.get_item().get_estimation());
        }
    }

    // TODO: Could be more efficient to calculate the score for each new node
    // and then sort the resulting Vec<Node> according this score.
    pub(crate) fn node_generator(&self, parent: &Node, maximazing: bool) -> Vec<Node> {
        let parent_goban = parent.get_item();
        let parent_player = parent_goban.get_player();
        let parent_enemy = parent_goban.get_enemy();
        let parent_player_captures = parent.get_player_captures();
        let parent_enemy_captures = parent.get_opponent_captures();

        // TODO: Investigate this call and its return value (especially for open 2).
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
                Node::new(Goban::new_with_estimation(player, enemy), parent.get_depth() + 1, *b, is_players_move, player_captures, enemy_captures)
            })
            .collect();
        Self::sort_by_estimate(&mut ret);
        ret
    }
}
