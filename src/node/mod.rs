use std::{
    cell::RefCell,
    cmp::{Eq, Ordering},
    collections::BinaryHeap,
    fmt,
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::Rc
};

use crate::{
    bitboard::{BitBoard, pattern::*},
    goban::Goban
};
use crate::goban::fscore::Fscore;

#[cfg(test)]
mod tests;

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
/// `item` is the inner value which is held by a Node.
///
/// `branches` is a [`BinaryHeap`], wrapped in an [`Option`], which hold child nodes.
/// The type `Branches` is used for convenience and is just an alias for `BinaryHeap<Rc<RefCell<Node>>>`.
///
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Debug, Clone, Default)]
pub struct Node {
    /// `item` is the inner value which is held by a Node.
    item: Goban,
    depth: usize,
    is_computers_move: bool,
    last_move: BitBoard,
    computer_captures: u8,
    human_captures: u8,
    is_computer_threatened: Option<bool>,
    /// `branches` is a [`BinaryHeap`], wrapped in an [`Option`], which hold child nodes.
    /// The type `Branches` is used for convenience and is just an alias for `BinaryHeap<Rc<RefCell<Node>>>`.
    branches: Option<Branches>
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
    pub fn new(item: Goban, depth: usize, last_move: BitBoard, is_computers_move: bool, computer_captures: u8, human_captures: u8) -> Self {
        Self {
            item,
            depth,
            last_move,
            is_computers_move,
            computer_captures,
            human_captures,
            is_computer_threatened: None,
            branches: None
        }
    }

    pub fn get_item(&self) -> &Goban {
        &self.item
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_last_move(&self) -> BitBoard {
        self.last_move
    }

    pub fn get_computer_captures(&self) -> u8 {
        self.computer_captures
    }

    pub fn get_human_captures(&self) -> u8 {
        self.human_captures
    }

    pub fn is_computers_last_move(&self) -> bool {
        self.is_computers_move
    }

    pub fn compute_immediate_threats_for_computer(&mut self, patterns: &NewPattern) {
        let goban = self.get_item();
        let (computer, human) = (*goban.get_computer(), *goban.get_human());

        // let threats = extract_missing_bit_cross_three_with_four(*human, *human);

        // self.is_computer_threatened = Some((threats | extract_missing_bit_cross_four_with_four(*human, *computer)).is_any());
        self.is_computer_threatened = Some(extract_threatening_moves_from_computer(computer, human, self.human_captures, patterns).is_any());
    }

    pub fn is_computer_threatened(&self) -> bool {
        if let Some(is_threatened) = self.is_computer_threatened {
            return is_threatened;
        }

        true
    }

    pub fn set_item_fscore(&mut self, fscore: Fscore) {
        self.item.set_fscore(fscore);
    }

    pub fn compute_item_fscore(
        &mut self,
        previous_state: &Goban,
        to_play: &BitBoard,
        depth: usize
    ) -> Fscore {
        self.item.compute_fscore(previous_state, to_play, depth)
    }

    pub fn add_branch(&mut self, item: Goban, last_move: BitBoard, is_computers_move: bool) -> Rc<RefCell<Self>> {
        let new_node = Rc::new(RefCell::new(Self::new(item, self.depth + 1, last_move, is_computers_move, self.computer_captures, self.human_captures)));
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
    pub fn add_many_branches(&mut self, new_branches: Vec<Node>) {
        let mut new_branches: BinaryHeap<Rc<RefCell<Node>>> = new_branches
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();

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
