use std::rc::Rc;

use super::{
    tree::{Tree, node::Node},
    goban::{Goban, Fscore},
    bitboard::BitBoard
};

pub struct Algorithm
{
    play_tree: Node,
}

impl Algorithm
{
    pub fn new(initial_state: Goban) -> Self {
        let play_tree = Node::new(initial_state, 0);
        Algorithm {
            play_tree
        }
    }

    // TODO: There is a lot of duplicated code in this function, we should refactor it.
    fn minimax(node: &mut Node, depth: u32, maximazing: bool) -> Node {
        let current_goban = node.get_item().clone();
        if depth == 0 {
            node.compute_item_fscore(&current_goban, &BitBoard::empty(), depth as usize);
            return node.clone();
        }
        let mut candidate = node.clone();
        let mut fscore = node.get_item().get_fscore();
        if fscore.is_win() {
            return candidate;
        }

        if maximazing {
            fscore = Fscore::Value(isize::MIN);
            node.add_many_branches(Self::node_generator);
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild = Self::minimax(&mut child.borrow_mut(), depth - 1, !maximazing);
                    let grandchild_fscore = grandchild.get_item().get_fscore();
                    child.borrow_mut().set_item_fscore(grandchild_fscore);
                    if fscore < grandchild_fscore {
                        candidate = child.borrow().clone();
                        fscore = grandchild_fscore;
                    }
                }
            }
        }
        else {
            fscore = Fscore::Value(isize::MAX);
            node.add_many_branches(Self::node_generator);
            let children = node.get_branches();
            if let Some(children) = children {
                for child in children {
                    let grandchild = Self::minimax(&mut child.borrow_mut(), depth - 1, !maximazing);
                    let grandchild_fscore = grandchild.get_item().get_fscore();
                    child.borrow_mut().set_item_fscore(grandchild_fscore);
                    if fscore > grandchild_fscore {
                        candidate = child.borrow().clone();
                        fscore = grandchild_fscore;
                    }
                }
            }
        }

        candidate
    }

    fn node_generator(parent: &mut Node) -> Vec<Node> {
        parent
            .get_item()
            .list_neighbours()
            .enumerate()
            .iter()
            .map(|b| Node::new(Goban::new(parent.get_item().get_player() | b, *parent.get_item().get_enemy()), parent.get_depth() + 1))
            .collect()
    }

    /// This mehtod is likely to change in a near future because I'm not sure what to return.
    /// For now it returns a BitBoard that contains the next move to play.
    pub fn get_next_move(&mut self, maximazing: bool) -> Option<BitBoard> {
        let next_state = Self::minimax(&mut self.play_tree, 3, maximazing);
        if next_state == self.play_tree {
            None
        } else {
            Some(next_state.get_item().get_player() ^ self.play_tree.get_item().get_player())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::goban::Goban;
    use crate::bitboard::BitBoard;
    use crate::algorithm::Algorithm;
    use crate::tree::node::Node;

    #[test]
    fn test_algorithm()
    {
        let (mut player, mut enemy) = (BitBoard::default(), BitBoard::from_str("
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000001000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
            0000000000000000000
        "));
        let initial = Goban::new(player, enemy);
        let mut algo = Algorithm::new(initial);

        for _ in 0..10 {
            let next_move = algo.get_next_move(true);
            if next_move.is_none() { break; }
            let next_move = next_move.unwrap();
            println!("Here is the next move to play for player:\n{}", next_move);
            player |= next_move;
            println!("Player's BitBoard:\n{}", player);
            let initial = Goban::new(enemy, player);
            algo = Algorithm::new(initial);
            let next_move = algo.get_next_move(true);
            if next_move.is_none() { break; }
            let next_move = next_move.unwrap();
            println!("Here is the next move to play for enemy:\n{}", next_move);
            enemy |= next_move;
            println!("Enemy's BitBoard:\n{}", enemy);
            let initial = Goban::new(player, enemy);
            algo = Algorithm::new(initial);
        }
        todo!();
    }

    #[test]
    fn generic_algo_test()
    {
        let to_play = BitBoard::from_str("
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000001000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		0000000000000000000
		");

        // let tree = Algorithm::new(Goban::get_heuristic, board);
        let board = Goban::new(to_play, BitBoard::default());
        let mut node = Node::new(board, 0);
        let result = Algorithm::minimax(&mut node, 3, true);
        println!("Here is what to got:\n{}", result);
        assert_eq!(1, 2 + 2);
    }
}