use tree::binary_tree::{ BinaryTree, Iter };

pub struct Set<E: Eq + Ord + Clone> {
    tree: BinaryTree<E>
}

impl<E: Eq + Ord + Clone> Set<E> {
    pub fn new(&self) -> Self {
        Set {
            tree: BinaryTree::empty()
        }
    }
}
