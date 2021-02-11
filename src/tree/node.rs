use std::{
    rc::Rc,
    cell::RefCell,
    fmt::Debug,
    collections::BinaryHeap,
    cmp::{Ordering, Eq},
    hash::{Hash, Hasher},
    fmt
};

use crate::{bitboard::BitBoard, goban::{Fscore, Goban}};

/// This type is an alias for `BinaryHeap<Rc<RefCell<Node>>>`.
pub type Branches = BinaryHeap<Rc<RefCell<Node>>>;

/// The struct that represent a node in a tree.
/// ```
/// pub struct Node
/// {
///     item: Goban,
///     branches: Option<Branches>
/// } 
/// ```
/// `item` is the inner value which is holded by a Node.
/// 
/// `branches` is a [`BinaryHeap`], wrapped in an [`Option`], which hold child nodes.
/// The type `Branches` is used for convenience and is just an alias for `BinaryHeap<Rc<RefCell<Node>>>`.
/// 
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Debug, Clone, Default)]
pub struct Node
{
    /// `item` is the inner value which is holded by a Node.
    item: Goban,
    depth: usize,
    last_move: BitBoard,
    /// `branches` is a [`BinaryHeap`], wrapped in an [`Option`], which hold child nodes.
    /// The type `Branches` is used for convenience and is just an alias for `BinaryHeap<Rc<RefCell<Node>>>`.
    branches: Option<Branches>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inner: {}\nBranches: {:#?}", self.item, self.branches)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
    }
}

impl Node {
    pub fn new(item: Goban, depth: usize, last_move: BitBoard) -> Self {
        Self {
            item,
            depth,
            last_move,
            branches: None
        }
    }

    pub fn get_item(&self) -> &Goban {
        &self.item
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn set_item_fscore(&mut self, fscore: Fscore) {
        self.item.set_fscore(fscore);
    }

    pub fn compute_item_fscore(&mut self, previous_state: &Goban, to_play: &BitBoard, depth: usize) -> Fscore {
        self.item.compute_fscore(previous_state, to_play, depth)
    }

    pub fn add_branch(&mut self, item: Goban, last_move: BitBoard) -> Rc<RefCell<Self>> {
        let new_node = Rc::new(RefCell::new(Self::new(item, self.depth + 1, last_move)));
        let mut branches = self.branches.take().unwrap_or_default();

        branches.push(Rc::clone(&new_node));
        self.branches.replace(branches);

        new_node
    }

    /// This method should not have any overhead as the method len()
    /// actually calls the same method on the underlying type (which is a Vec)
    /// which is just a getter on the len property on Vec
    pub fn count_branch(&self) -> usize {
        if let Some(ref branches) = self.branches {
            branches.len()
        } else {
            0
        }
    }

    /// A method that add many branches at once using the closure `generator`.
    pub fn add_many_branches<F: Fn(&mut Self, bool) -> Vec<Node>>(&mut self, generator: F, maximizing: bool) {
        let mut new_branches: BinaryHeap<Rc<RefCell<Node>>> = generator(self, maximizing).into_iter().map(|x| Rc::new(RefCell::new(x))).collect();

        if !new_branches.is_empty() {
            let mut branches = self.branches.take().unwrap_or_default();
            branches.append(&mut new_branches);
            self.branches = Some(branches);
        }
    }

    // TODO: Ideally, this method should returns an Iterator (not an option)
    // in order to be able to directly iterate over its return value.
    /// Returns the Branches of the current node, if any, wrapped into an Option.
    /// Returns None otherwise.
    pub fn get_branches(&mut self) -> Option<&Branches> {
        if let Some(ref branches) = self.branches {
            Some(branches)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        rc::Rc,
        cell::RefCell,
        collections::{BinaryHeap}
    };
    use crate::{bitboard::BitBoard, goban::Goban};

    use super::{Node, Branches};

    #[test]
    #[allow(unused)]
    fn add_many_branches_with_valid_node_generator_should_add_branches()
    {
        // Arrange
        let closure = |n: &mut Node, m| {
            let mut vec = Vec::new();
            for i in 1..10 {
                let mut bitboard = BitBoard::default();
                bitboard.set(i, true);
                vec.push(Node::new(Goban::new(bitboard, bitboard), n.depth + 1, BitBoard::empty()));
            }
            vec
        };
        let mut node = Node::new(Goban::default(), 0, BitBoard::empty());

        // Act
        node.add_many_branches(closure, true);
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 9);
    }

    #[test]
    fn count_branch_with_0_branch_should_return_0() {
        // Arrange
        let bitboard = BitBoard::default();
        let node = Node {
            item: Goban::new(bitboard, bitboard),
            depth: 0,
            last_move: BitBoard::empty(),
            branches: Some(Branches::new())
        };

        // Act
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 0);
    }

    #[test]
    fn count_branch_with_no_branch_should_return_0() {
        // Arrange
        let bitboard = BitBoard::default();
        let node = Node {
            item: Goban::new(bitboard, bitboard),
            depth: 0,
            last_move: BitBoard::empty(),
            branches: None
        };

        // Act
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 0);
    }

    #[test]
    fn count_branch_with_3_branches_should_return_3() {
        // Arrange
        let bitboards = [BitBoard::full(), BitBoard::empty()];
        let (node0, node1, node2) = (
            Rc::new(RefCell::new(Node {
                item: Goban::new(bitboards[0], bitboards[1]),
                depth: 1,
                last_move: BitBoard::empty(),
                branches: None
            })),
            Rc::new(RefCell::new(Node {
                item: Goban::new(bitboards[1], bitboards[0]),
                depth: 1,
                last_move: BitBoard::empty(),
                branches: None
            })),
            Rc::new(RefCell::new(Node {
                item: Goban::new(bitboards[0], bitboards[0]),
                depth: 1,
                last_move: BitBoard::empty(),
                branches: None
            }))
        );
        let mut branches = BinaryHeap::new();
        branches.push(node0);
        branches.push(node1);
        branches.push(node2);
        let node = Node {
            item: Goban::new(bitboards[1], bitboards[1]),
            depth: 0,
            last_move: BitBoard::empty(),
            branches: Some(branches)
        };

        // Act
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 3);
    }

    #[test]
    fn add_branch_should_add_a_branch() {
        // Arrange
        let bitboards = [BitBoard::full(), BitBoard::empty()];
        let mut node = Node {
            item: Goban::new(bitboards[0], bitboards[1]),
            depth: 0,
            last_move: BitBoard::empty(),
            branches: None
        };

        // Act
        let new_node = node.add_branch(Goban::new(bitboards[1], bitboards[0]), BitBoard::empty());
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 1);
        assert_eq!(node.branches.unwrap().peek().unwrap(), &new_node);
    }

    #[test]
    #[ignore]
    fn test_display_no_assert() {
        // Arrange
        let closure = |n: &mut Node, m| {
            let mut vec = Vec::new();
            for i in 1..10 {
                let mut bitboard = BitBoard::default();
                bitboard.set(i, true);
                vec.push(Node::new(Goban::new(bitboard, bitboard), n.depth + 1, BitBoard::empty()));
            }
            vec
        };
        let mut node = Node::new(Goban::default(), 0, BitBoard::empty());

        // Act
        node.add_many_branches(closure, true);
        println!("Here is a node with 9 branches:\n{}", node);
    }
}