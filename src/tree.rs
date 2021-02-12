pub(crate) mod node;

use node::Node;
use crate::{bitboard::BitBoard, goban::Goban};

pub struct Tree
{
    // The heuristic function to use.
    // This is a generic because it allows us to mock the behavior of the heuristic function in tests.
    depth: u64,
    root: Node
}

impl Tree
{
    pub fn new(root_item: Goban) -> Self {
        Self {
            depth: 0,
            root: Node::new(root_item, 0, BitBoard::empty(), 0, 0)
        }
    }
}