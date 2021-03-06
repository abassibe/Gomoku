use crate::algorithm::{Algorithm, transposition_table::*};
use crate::goban::fscore::Fscore;
use crate::node::Node;

impl Algorithm {
    pub(super) fn minimax(&self, node: &mut Node, depth: u32, mut alpha: Fscore, mut beta: Fscore, maximizing: bool) -> Node {
        if depth == 0 || self.is_game_over(node) {
            match tt_lookup_state(&node.get_item())
            {
                Some(fscore) => {
                    node.set_item_fscore(fscore);
                },
                None => {
                    self.compute_and_set_fscore(node, depth + 1);
                    tt_insert_new_state(*node.get_item(), node.get_item().get_fscore());
                }
            }
            return node.clone();
        }
        let mut candidate = node.clone();

        if maximizing {
            let mut fscore = Fscore::Value(isize::MIN);
            node.add_many_branches(self.node_generator(&node, maximizing));

            if let Some(children) = node.get_branches() {

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
            if let Some(children) = node.get_branches() {
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
