enum Color {
    Black,
    Red
}

pub enum Tree<E: Clone> {
    Empty,
    Node(Color, Rc<Tree>, E, Rc<Tree>),
}

impl<E: Clone> Tree<E> {
    pub fn node(Color _c, E val, Tree left, Tree right) {

    }
}
