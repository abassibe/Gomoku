use std::{rc::Rc, cell::RefCell};

use super::{
    tree::{Tree, node::Node},
    goban::Goban
};

pub struct Algorithm<H: Fn(Goban) -> u64>
{
    play_tree: Tree<H, Goban>,
}

impl<H: Fn(Goban) -> u64> Algorithm<H>
{
    const HEURISTIC_WIN_VALUE: u64 = u64::MAX - 1;

    pub fn new(heuristic: H, initial_state: Goban) -> Self {
        let play_tree = Tree::new(heuristic, initial_state);
        Algorithm {
            play_tree
        }
    }

    fn minimax(node: &mut Node<Goban>, depth: u32, maximazing: bool) -> usize {
        let mut fscore = node.get_item().get_fscore();
        if depth == 0 || fscore as u64 == Self::HEURISTIC_WIN_VALUE {
            return fscore;
        }
        if maximazing {
            fscore = usize::MIN;
            node.add_many_branches(Self::node_generator);
            let children = node.get_branches();
            if children.is_some() {
                for n in children.unwrap() {
                    fscore = fscore.max(Self::minimax(&mut n.borrow_mut(), depth - 1, false));
                }
            }
        }
        else {
            fscore = usize::MAX;
            node.add_many_branches(Self::node_generator);
            let children = node.get_branches();
            if children.is_some() {
                for n in children.unwrap() {
                    fscore = fscore.min(Self::minimax(&mut n.borrow_mut(), depth - 1, true));
                }
            }
        }

        fscore
    }

    fn node_generator(parent: &mut Node<Goban>) -> Vec<Node<Goban>> {
        parent
            .get_item()
            .list_neighbours()
            .enumerate()
            .iter()
            .map(|b| Node::new(Goban::new(parent.get_item().get_player() | b, *parent.get_item().get_enemy())))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::goban::Goban;
    use crate::algorithm::Algorithm;
    use crate::bitboard::BitBoard;

    #[test]
    fn generic_algo_test()
    {
        let board = Goban::default();
        let tree = Algorithm::new(Goban::get_heuristic());
    }
}