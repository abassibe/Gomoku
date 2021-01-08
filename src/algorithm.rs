use super::{
    tree::{Tree, node::Node},
    goban::Goban,
    bitboard::BitBoard
};

pub struct Algorithm
{
    play_tree: Tree,
}

impl Algorithm
{
    pub const HEURISTIC_WIN_VALUE: u64 = u64::MAX - 1;

    pub fn new(initial_state: Goban) -> Self {
        let play_tree = Tree::new(initial_state);
        Algorithm {
            play_tree
        }
    }

    fn minimax(node: &mut Node, depth: u32, maximazing: bool) -> usize {
        let mut fscore = node.get_item().get_fscore();
        // println!("What am I doing\n{}", node.get_item());
        if depth == 0 || fscore as u64 == Self::HEURISTIC_WIN_VALUE {
            return fscore;
        }
        if maximazing {
            fscore = usize::MIN;
            node.add_many_branches(Self::node_generator);
            let children = node.get_branches();
            if children.is_some() {
                for n in children.unwrap() {
                    fscore = fscore.max(Self::minimax(&mut n.borrow_mut(), depth - 1, !maximazing));
                }
            }
        }
        else {
            fscore = usize::MAX;
            node.add_many_branches(Self::node_generator);
            let children = node.get_branches();
            if children.is_some() {
                for n in children.unwrap() {
                    fscore = fscore.min(Self::minimax(&mut n.borrow_mut(), depth - 1, !maximazing));

                }
            }
        }

        fscore
    }

    fn node_generator(parent: &mut Node) -> Vec<Node> {
        parent
            .get_item()
            .list_neighbours()
            .enumerate()
            .iter()
            .map(|b| {
                let mut goban = Goban::new(parent.get_item().get_player() | b, *parent.get_item().get_enemy());
                goban.compute_fscore(parent.get_item(), b, parent.get_depth() + 1);
                Node::new(goban)
            })
            .collect()
    }

    /// This mehtod is likely to change in a near future because I'm not sure what to return.
    /// For now it returns a BitBoard that contains the next move to play.
    pub fn get_next_move(&mut self, maximazing: bool) -> BitBoard {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::goban::Goban;
    use crate::bitboard::BitBoard;
    use crate::algorithm::Algorithm;
    use crate::tree::node::Node;

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
        let mut node = Node::new(board);
        let result = Algorithm::minimax(&mut node, 3, true);
        assert_eq!(1, 2 + 2);
    }
}