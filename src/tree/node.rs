use std::{
    rc::Rc,
    cell::RefCell,
    fmt::Debug,
    collections::BinaryHeap,
    cmp::{Ordering, Eq},
    hash::{Hash, Hasher},
    fmt
};

/// This type is an alias for `BinaryHeap<Rc<RefCell<Node<T>>>>`.
pub type Branches<T> = BinaryHeap<Rc<RefCell<Node<T>>>>;

/// The struct that represent a node in a tree.
/// ```
/// pub struct Node<T>
/// {
///     item: T,
///     branches: Option<Branches<T>>
/// } 
/// ```
/// `item` is the inner value which is holded by a Node.
/// It could be of any type that implement the following traits:
/// * Display
/// * Debug
/// * Ord
/// * Eq
/// * Hash
/// 
/// `branches` is a [`BinaryHeap`], wrapped in an [`Option`], which hold child nodes.
/// The type `Branches` is used for convenience and is just an alias for `BinaryHeap<Rc<RefCell<Node<T>>>>`.
/// `T` has to be of the same type as `item`.
/// 
/// [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Debug)]
pub struct Node<T>
{
    /// `item` is the inner value which is holded by a Node.
    /// It could be of any type that implement the following traits:
    /// * Display
    /// * Debug
    /// * Ord
    /// * Eq
    /// * Hash
    item: T,
    depth: usize,
    /// `branches` is a [`BinaryHeap`], wrapped in an [`Option`], which hold child nodes.
    /// The type `Branches` is used for convenience and is just an alias for `BinaryHeap<Rc<RefCell<Node<T>>>>`.
    /// `T` has to be of the same type as `item`.
    branches: Option<Branches<T>>,
}

impl<T: fmt::Display + Debug> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inner: {}\nBranches: {:#?}", self.item, self.branches)
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl<T: Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: Eq> Eq for Node<T> {}

impl<T: Hash> Hash for Node<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
    }
}

impl<T: Ord> Node<T> {
    pub fn new(item: T) -> Self {
        Self {
            item,
            depth: 0,
            branches: None
        }
    }

    pub fn add_branch(&mut self, item: T) -> Rc<RefCell<Self>> {
        let new_node = Rc::new(RefCell::new(Self::new(item)));
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
    pub fn add_many_branches<U: Iterator<Item = Node<T>>, F: Fn(&mut Self) -> U>(&mut self, generator: F) {
        let mut new_branches: BinaryHeap<Rc<RefCell<Node<T>>>> = generator(self).map(|x| Rc::new(RefCell::new(x))).collect();

        if !new_branches.is_empty() {
            let mut branches = self.branches.take().unwrap_or_default();
            branches.append(&mut new_branches);
            self.branches = Some(branches);
        }
    }

    // TODO: Ideally, this method should returns an Iterator (not an option)
    // in order to be able to directly iterate over its return value.
    /// Returns an iterator which iterates over the branches in the current node.
    pub fn get_branches(&mut self) -> Option<impl Iterator<Item = &Rc<RefCell<Node<T>>>> + '_> {
        if let Some(ref branches) = self.branches {
            Some(branches.iter())
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
    use super::{Node, Branches};

    #[test]
    #[allow(unused)]
    fn add_many_branches_with_valid_node_generator_should_add_branches()
    {
        // Arrange
        let closure = |x: &mut Node<u32>| {
            let mut vec = Vec::new();
            for i in 1..10 {
                vec.push(Node::new(i));
            }
            vec.into_iter()
        };
        let mut node = Node::new(0);

        // Act
        node.add_many_branches(closure);
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 9);
    }

    #[test]
    fn count_branch_with_0_branch_should_return_0() {
        // Arrange
        let node = Node {
            item: 42,
            depth: 0,
            branches: Some(Branches::<u32>::new())
        };

        // Act
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 0);
    }

    #[test]
    fn count_branch_with_no_branch_should_return_0() {
        // Arrange
        let node = Node {
            item: 42,
            depth: 0,
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
        let (node0, node1, node2) = (
            Rc::new(RefCell::new(Node {
                item: 0,
                depth: 1,
                branches: None
            })),
            Rc::new(RefCell::new(Node {
                item: 1,
                depth: 1,
                branches: None
            })),
            Rc::new(RefCell::new(Node {
                item: 2,
                depth: 1,
                branches: None
            }))
        );
        let mut branches = BinaryHeap::new();
        branches.push(node0);
        branches.push(node1);
        branches.push(node2);
        let node = Node {
            item: 42,
            depth: 0,
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
        let mut node = Node {
            item: 42,
            depth: 0,
            branches: None
        };

        // Act
        let new_node = node.add_branch(84);
        let nb_branches = node.count_branch();

        // Assert
        assert_eq!(nb_branches, 1);
        assert_eq!(node.branches.unwrap().peek().unwrap(), &new_node);
    }

    #[test]
    fn test_display_no_assert() {
        // Arrange
        let closure = |x: &mut Node<u32>| {
            let mut vec = Vec::new();
            for i in 1..10 {
                vec.push(Node::new(i));
            }
            vec.into_iter()
        };
        let mut node = Node::new(0);

        // Act
        node.add_many_branches(closure);
        println!("Here is a node with 9 branches:\n{}", node);
    }
}