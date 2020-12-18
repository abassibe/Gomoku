pub(crate) mod node;

use node::Node;

pub struct Tree<H, T>
where
    H: Fn(T) -> u64,
    T: Ord
{
    // The heuristic function to use.
    // This is a generic because it allows us to mock the behavior of the heuristic function in tests.
    heuristic: H,
    depth: u64,
    root: Node<T>
}

impl<H, T> Tree<H, T>
where
    H: Fn(T) -> u64,
    T: Ord
{
    pub fn new(heuristic: H, root_item: T) -> Self {
        Self {
            heuristic,
            depth: 0,
            root: Node::new(root_item)
        }
    }
}