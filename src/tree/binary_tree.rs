use std::cmp::max;
use std::fmt::{Debug, Formatter, Result};
use std::iter::{Iterator, IntoIterator, FromIterator};
use std::ops::Deref;
use std::rc::Rc;
use std::borrow::Borrow;


use list::List;

#[macro_escape]
#[macro_export]
macro_rules! binary_tree {
    [] => {BinaryTree::Empty};
    [$($x:expr),*] => {{
        let mut t = BinaryTree::empty();
        $(
            t = t.insert($x);
        )*
        t
    }};
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Color {
    NegativeBlack,
    DoubleBlack,
    Black,
    Red
}

#[derive(Clone, Eq)]
pub enum BinaryTree<E> {
    Empty,
    Node(Color, E, Rc<BinaryTree<E>>, Rc<BinaryTree<E>>),
}

pub struct Iter<E: Clone> {
    stack: Rc< List< Rc< BinaryTree<E> > > >,
}

impl<E> BinaryTree<E> where E: Clone + Eq + Ord {
    #[inline]
    pub fn node(_c: Color, val: E, left: BinaryTree<E>, right: BinaryTree<E>) -> Self {
        //assert!(left.is_empty() || left.value() < val);
        //assert!(right.is_empty() || right.value() > val);
        BinaryTree::Node(_c, val, Rc::new(left), Rc::new(right))
    }

    #[inline]
    pub fn empty() -> Self {
        BinaryTree::Empty
    }

    #[inline]
    pub fn unsafe_get<K>(&self, val: K) -> E where E:PartialOrd<K> {
        match self.get(val) {
            None => panic!("Value not found for unsafe_get call"),
            Some(value) => value,
        }
    }

    pub fn get<K>(&self, val: K) -> Option<E> where E: PartialOrd<K> {
        match self {
            &BinaryTree::Empty => None,
            &BinaryTree::Node(_, ref value, ref left, ref right) =>
                if value > &val { left.get(val) }
                else if value < &val { right.get(val) }
                else { Some(value.clone()) }
        }
    }

    pub fn insert(&self, val: E) -> Self {
        self.ins(val).paint(Color::Black)
    }

    fn ins(&self, val: E) -> Self {
        match self {
            &BinaryTree::Empty => BinaryTree::node(Color::Red, val, BinaryTree::Empty, BinaryTree::Empty),
            &BinaryTree::Node(color, ref root, ref left, ref right) => {
                if val > *root { BinaryTree::balance(color, root.clone(), left.deref().clone(), right.ins(val)) }
                else if val < *root { BinaryTree::balance(color, root.clone(), left.ins(val), right.deref().clone()) }
                else { self.clone() }
            }
        }
    }

    pub fn del(&self) -> Self {
        //Leaf case
        if self.is_leaf() { BinaryTree::Empty }
        //One child cases
        else if self.left().is_empty() { self.left().deref().clone() }
        else if self.right().is_empty() { self.right().deref().clone() }
        //Two child case
        else {
            match self.inorder_successor() {
                BinaryTree::Empty =>
                    panic!("This should not occur, right.is_empty() check is not working"),

                BinaryTree::Node(color, ref root, ref left, ref right) =>
                    BinaryTree::Node(
                        color,
                        root.clone(),
                        left.clone(),
                        Rc::new(right.delete(root)))
            }
        }
    }

    pub fn delete<B: Borrow<E>>(&self, val: B) -> Self {
        match self {
            &BinaryTree::Empty => BinaryTree::Empty,
            &BinaryTree::Node(ref color, ref root, ref left, ref right) => {
                if root > val.borrow() {
                    BinaryTree::Node(
                        color.clone(),
                        root.clone(),
                        Rc::new(left.delete(val)),
                        right.clone())
                }
                else if root < val.borrow() {
                    BinaryTree::Node(
                        color.clone(),
                        root.clone(),
                        left.clone(),
                        Rc::new(right.delete(val)))
                } else {
                    self.del()
                }
            }
        }
    }


    pub fn inorder_successor(&self) -> Self {
        if self.right().is_empty() { BinaryTree::Empty }
        else { self.right().minimum() }
    }

    pub fn minimum(&self) -> Self {
        if self.left().is_empty() { self.clone() }
        else { self.left().minimum() }
    }
    /*pub fn delete(&self, val: E) -> Self {
        self.del(val).paint(Color::Black)
    }

    fn del(&self, val: E) -> Self {
        match self {
            &BinaryTree::Empty => BinaryTree::Empty,
            &BinaryTree::Node(color, ref root, ref left, ref right) => {
                if val > *root { BinaryTree::balance(color, root.clone(), left.deref().clone(), right.rmv(val)) }
                else if val < *root { BinaryTree::balance(color, root.clone(), left.rmv(val), right.deref().clone()) }
                else { self.delete() }
            }
        }
    }

    fn remove(&self) -> Self {
        match self {
            &BinaryTree::Empty => BinaryTree::Empty,
            //Leaf node
            &BinaryTree::Node(color, _, BinaryTree::Empty, BinaryTree::Empty) => BinaryTree::Empty,
            //One child node
            &BinaryTree::Node(color, _, ref left, BinaryTree::Empty) => left.deref().clone(),
            &BinaryTree::Node(color, _, BinaryTree::Empty, ref right) => right.deref().clone(),
            &BinaryTree::Node(color, _, ref left, ref right) => BinaryTree::Node(color, right.left_most(), left.clone(), right.clone()),
        }
    }*/

    fn balance(c:Color, val:E, left: BinaryTree<E>, right: BinaryTree<E>) -> Self {
        match c {
            Color::Red => BinaryTree::node(c, val, left, right),
            Color::Black => {
                if left.doubled_left() {
                    //println!("left double_left\n{:?}", right);
                    BinaryTree::node(Color::Red
                        , left.value()
                        , left.left().paint(Color::Black)
                        , BinaryTree::Node(Color::Black, val, left.right(), Rc::new(right))
                        )
                } else if left.doubled_right() {
                    //println!("left double_right\n{:?}", right);
                    BinaryTree::node(Color::Red
                        , left.right().value()
                        , BinaryTree::Node(Color::Black, left.value(), left.left(), left.right().left())
                        , BinaryTree::Node(Color::Black, val, left.right().right(), Rc::new(right))
                        )
                } else if right.doubled_left() {
                    //println!("right double_left\n{:?}", right);
                    BinaryTree::node(Color::Red
                        , right.left().value()
                        , BinaryTree::Node(Color::Black, val, Rc::new(left), right.left().left())
                        , BinaryTree::Node(Color::Black, right.value(), right.left().right(), right.right())
                        )
                } else if right.doubled_right() {
                    //println!("right double_right\n{:?}", right);
                    BinaryTree::node(Color::Red
                        , right.value()
                        , BinaryTree::Node(Color::Black, val, Rc::new(left), right.left())
                        , right.right().paint(Color::Black)
                        )
                } else {
                    BinaryTree::node(c, val, left, right)
                }
            },
            _ => panic!("NegativeBlack or DoubleBlack left in tree, should've been removed by delete"),
        }
    }

    pub fn doubled_left(&self) -> bool {
            !self.is_empty()
        &&  self.color() == Color::Red
        &&  !self.left().is_empty()
        &&  self.left().color() == Color::Red
    }
    pub fn doubled_right(&self) -> bool {
            !self.is_empty()
        &&  self.color() == Color::Red
        &&  !self.right().is_empty()
        &&  self.right().color() == Color::Red
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &BinaryTree::Empty => true,
            &BinaryTree::Node(_, _, _, _) => false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            &BinaryTree::Empty => false,
            &BinaryTree::Node(_, _, ref left, ref right) => left.is_empty() && right.is_empty(),
        }
    }

    pub fn paint(&self, c:Color) -> Self {
        match self {
            &BinaryTree::Empty => panic!("paint() called on empty tree"),
            &BinaryTree::Node(_, ref value, ref left, ref right) => BinaryTree::Node(c, value.clone(), left.clone(), right.clone()),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            &BinaryTree::Empty => panic!("color() called on empty tree"),
            &BinaryTree::Node(color, _, _, _) => color,
        }
    }
    pub fn value(&self) -> E {
        match self {
            &BinaryTree::Empty => panic!("value() called on empty tree"),
            &BinaryTree::Node(_, ref value, _, _) => value.clone(),
        }
    }
    pub fn left(&self) -> Rc<BinaryTree<E>> {
        match self {
            &BinaryTree::Empty => panic!("left() called on empty tree"),
            &BinaryTree::Node(_, _, ref left, _) => left.clone(),
        }
    }
    pub fn right(&self) -> Rc<BinaryTree<E>> {
        match self {
            &BinaryTree::Empty => panic!("right() called on empty tree"),
            &BinaryTree::Node(_, _, _, ref right) => right.clone(),
        }
    }
    pub fn length(&self) -> usize {
        match self {
            &BinaryTree::Empty => 0,
            &BinaryTree::Node(_, _, ref left, ref right) => 1 + left.length() + right.length(),
        }
    }
    pub fn height(&self) -> usize {
        match self {
            &BinaryTree::Empty => 0,
            &BinaryTree::Node(_, _, ref left, ref right) => 1 + max(left.height(), right.height()),
        }
    }
}

impl<E:PartialEq> PartialEq for BinaryTree<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&BinaryTree::Empty, &BinaryTree::Empty) =>
                true,

            (_, &BinaryTree::Empty) =>
                false,

            (&BinaryTree::Empty, _) =>
                false,

            (&BinaryTree::Node(_, ref a_root, ref a_left, ref a_right), &BinaryTree::Node(_, ref b_root, ref b_left, ref b_right)) =>
                (a_root == b_root) && (a_left == b_left) && (a_right == b_right)
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }

}

impl<E: Clone + Debug> Debug for BinaryTree<E>{
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &BinaryTree::Empty => write!(f, "Empty"),
            &BinaryTree::Node(ref color, ref value, ref left, ref right) => write!(f, "Node({:?}, {:?}, {:?}, {:?})", color, value.clone(), left.clone(), right.clone()),
        }

    }
}


impl<E: Clone + Ord + Eq> IntoIterator for BinaryTree<E> {
    type Item = E;
    type IntoIter = Iter<E>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let root = Rc::new(self);
        Iter {
            stack: Rc::new(list![root.clone()]),
        }
    }
}

impl<E: Clone + Ord + Eq> FromIterator<E> for BinaryTree<E> {
    fn from_iter<I: IntoIterator<Item=E>>(iterator: I) -> Self {
        iterator
            .into_iter()
            .fold(BinaryTree::Empty, | tree, ele | tree.insert(ele))
    }
}

impl<E: Clone + Ord + Eq> Iterator for Iter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let node = self.stack.head();
        self.stack = self.stack.tail();
        if !node.right().is_empty() {
            self.stack = List::cons(node.right(), self.stack.clone()).into();
        }
        if !node.left().is_empty() {
            self.stack = List::cons(node.left(), self.stack.clone()).into();
        }
        return Some(node.value());
    }
}

#[test]
fn tree_macro() {
    let tree: BinaryTree<i32> =
        BinaryTree::empty()
            .insert(1)
            .insert(2)
            .insert(3)
            .insert(4)
            .insert(5)
            .insert(6);

    assert_eq!(binary_tree![1, 2, 3, 4, 5, 6], tree);
}

#[test]
fn test_height() {
    /*let tree: BinaryTree<i32> = (0..100000).collect();
    println!("{:?}", tree.height());*/
}

#[test]
fn tree_balance() {
    let tree_list: List<BinaryTree<i32>> =
        list![
            binary_tree![1, 2, 3]
            , binary_tree![1, 3, 2]
            , binary_tree![2, 1, 3]
            , binary_tree![2, 3, 1]
            , binary_tree![3, 1, 2]
            , binary_tree![3, 2, 1] ];

    for a in tree_list.clone().into_iter() {
        for b in tree_list.clone().into_iter() {
            assert_eq!(a, b);
        }
    }
}

#[test]
fn tree_iter() {
    let tree = binary_tree![1, 5, 3, 4, 2];
    let list: BinaryTree<i32> =
        tree.into_iter()
            .map(|x| x * 2)
            .collect();

    assert_eq!(list, binary_tree![8, 10, 4, 2, 6])
}
