pub(crate) mod node;

use node::Node;
use crate::goban::Goban;

pub struct Tree<H>
where
    H: Fn(Goban) -> u64,
{
    // The heuristic function to use.
    // This is a generic because it allows us to mock the behavior of the heuristic function in tests.
    heuristic: H,
    depth: u64,
    root: Node
}

impl<H> Tree<H>
where
    H: Fn(Goban) -> u64,
{
    pub fn new(heuristic: H, root_item: Goban) -> Self {
        Self {
            heuristic,
            depth: 0,
            root: Node::new(root_item)
        }
    }
}