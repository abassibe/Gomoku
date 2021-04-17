use crate::algorithm::Algorithm;
use crate::goban::fscore::Fscore;
use crate::node::{Branches, Node};

impl Algorithm {

    pub fn negamax(&self, node: &mut Node, depth: u32, mut alpha: Fscore, beta: Fscore) -> Node {
        if depth == 0 || self.is_game_over(node) {
            self.compute_and_set_fscore(node, depth + 1);
            return node.clone();
        }
        let mut candidate = node.clone();
        let mut fscore = Fscore::Value(isize::MIN);
        node.add_many_branches(self.node_generator(&node, true));
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
}