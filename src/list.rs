use std::fmt;
use std::iter::{Iterator, IntoIterator, FromIterator};
use std::vec::IntoIter;
use std::ops::Deref;
use std::rc::Rc;

macro_rules! list {
    [] => {List::Empty};
    [$ele:expr] => {List::cons($ele, List::Empty)};
    [$ele:expr, $($tail:expr),*] => {List::cons($ele, list![$($tail),*])};
}

#[derive(Clone, PartialEq, Eq)]
pub enum List<E> {
    Empty,
    Cons(E, Rc<List<E>>),
}

pub struct Iter<E: Clone> {
    node: Rc<List<E>>,
}

impl<E: Clone> List<E> {
    pub fn empty() -> List<E> {
        return List::Empty;
    }

    pub fn cons(head: E, tail: List<E>) -> List<E> {
        return List::Cons(head, Rc::new(tail));
    }

    pub fn head(&self) -> E {
        match self {
            &List::Empty => panic!("Head called on empty list"),
            &List::Cons(ref head, _) => head.clone(),
        }
    }

    pub fn tail(&self) -> Rc<List<E>> {
        match self {
            &List::Empty => panic!("Tail called on empy list"),
            &List::Cons(_, ref tail) => tail.clone(),
        }
    }

    pub fn safe_head(&self) -> Option<E> {
        match self {
            &List::Empty => None,
            &List::Cons(ref head, _) => Some(head.clone()),
        }
    }

    pub fn safe_tail(&self) -> Option<Rc<List<E>>> {
        match self {
            &List::Empty => None,
            &List::Cons(_, ref tail) => Some(tail.clone()),
        }
    }

    /*fn iter(&self) -> Iter<&E> {
        Iter {
            node: Rc::new(self)
        }
    }*/
}

impl<E:fmt::Display + Copy> fmt::Debug for List<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &List::Empty => write!(f, ""),
            &List::Cons(ref head, ref tail)  => write!(f, "{}, {:?}", head, tail),
        }
    }
}

impl<E: Clone> FromIterator<E> for List<E> {

    #[inline]
    fn from_iter<I: IntoIterator<Item=E>>(iterator: I) -> Self {
        iterator
            .into_iter()
            .fold(List::empty(), | lst, ele | List::cons(ele, lst))
    }
}

impl<E: Clone> Into<Rc<List<E>>> for List<E> {
    
}

impl<E: Clone> IntoIterator for List<E> {
    type Item = E;
    type IntoIter = Iter<E>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            node: Rc::new(self)
        }
    }
}

impl<E: Clone> Iterator for Iter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let tl;
        let hd;
        match self.node.deref() {
            &List::Empty => return None,
            &List::Cons(ref head, ref tail) => {
                tl = tail.clone();
                hd = head.clone();
            }
        }
        self.node = tl;
        Some(hd)
    }
}

#[test]
fn list_macro() {
    let lst: List<i32> = list![1, 2, 3];
    assert_eq!(lst, List::cons(1, List::cons(2, List::cons(3, List::empty()))));
}

#[test]
fn list_filter() {
    let lst: List<i32> = List::cons(3, List::cons(4, List::empty()));
    let lst2: List<i32> =
        lst.into_iter()
           .filter(|&x| x != 3)
           .collect();
    assert_eq!(lst2, List::cons(4, List::empty()));
}

#[test]
fn list_head() {
    //Equal
    let a: List<i32> = list![1];
    let b: List<i32> = list![1, 2];
    assert_eq!(a.head(), b.head());

    //Not Equal
    let a: List<i32> = list![1];
    let b: List<i32> = list![2, 1];
    assert!(a.head() != b.head());

    let a: List<i32> = list![];
    assert_eq!(a.head(), None);
}

#[test]
fn list_tail() {
    let a: List<i32> = list![1, 2, 3];
    assert_eq!(a.tail(), Some(Rc::new(list![2, 3])));

    let a: List<i32> = list![1, 2];
    let b: List<i32> = list![1, 2, 3];
    assert!(a.tail() != b.tail());
}
