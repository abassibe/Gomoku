use crate::algorithm::Algorithm;
use crate::goban::fscore::Fscore;
use crate::node::Node;

// Not sure if this is a good idea, just trying it out.
impl Algorithm {
    // TODO: There is a lot of duplicated code in this function, we should refactor it.
    pub(super) fn minimax(&self, node: &mut Node, depth: u32, mut alpha: Fscore, mut beta: Fscore, maximizing: bool) -> Node {
        if depth == 0 {
            // TODO: We have to pass the potential next move to compute_item_fscore, but we don't have it at this point
            // and I'm not even sure we actually need it, maybe we should remove it completely?
            // node.compute_item_fscore(&current_goban, current_goban.get_player(), depth as usize);
            self.compute_and_set_fscore(node);
            return node.clone();
        }
        let mut candidate = node.clone();
        let mut fscore = node.get_item().get_fscore();
        // if fscore.is_win() {
        if self.is_game_over(node) {
            self.compute_and_set_fscore(&mut candidate);
            return candidate;
        }

        if maximizing {
            fscore = Fscore::Value(isize::MIN);
            node.add_many_branches(self.node_generator(&node, maximizing));
            let children = node.get_branches();
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
            fscore = Fscore::Value(isize::MAX);
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
}
