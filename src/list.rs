use std::fmt;
use std::borrow::Borrow;
use std::convert::Into;
use std::iter::{Iterator, IntoIterator, FromIterator};
use std::ops::Deref;
use std::rc::Rc;
use std::vec::IntoIter;

#[macro_escape]
#[macro_export]
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

    #[inline]
    pub fn empty() -> List<E> {
        List::Empty
    }

    #[inline]
    pub fn cons<T: Borrow<List<E>>>(head: E, _tail: T) -> List<E> {
        let tail: Rc<List<E>> = _tail.borrow()
                                    .clone()
                                    .into();
        List::Cons(head, tail)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &List::Empty => true,
            &List::Cons(_, _) => false,
        }
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

    pub fn length(&self) -> usize {
        match self {
            &List::Empty => 0,
            &List::Cons(_, ref tail) => 1 + tail.length(),
        }
    }
    pub fn append<T: Borrow<List<E>>>(&self, lst: T) -> Self {
        match self {
            &List::Empty => lst.borrow().clone(),
            &List::Cons(ref head, ref tail) => List::Cons(head.clone(), tail.append(lst).into()),
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

impl<E:fmt::Debug + Clone> fmt::Debug for List<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &List::Empty => write!(f, ""),
            &List::Cons(ref head, ref tail)  => write!(f, "{:?}, {:?}", head.clone(), tail.clone()),
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

/*impl<E: Clone> Into<Rc<List<E>>> for List<E> {

    #[inline]
    fn into(self) -> Rc<List<E>> {
        Rc::new(self)
    }
}*/

impl<'a, E: Clone> IntoIterator for &'a List<E> {
    type Item = E;
    type IntoIter = Iter<E>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            node: Rc::new(self.clone())
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
fn list_iterator() {
    let lst: List<i32> = List::cons(3, List::cons(4, List::empty()));
    let lst2: List<String> =
        lst.clone()
           .into_iter()
           .filter(|&x| x != 3)
           .map(|x| x.to_string())
           .collect();

    assert_eq!(lst2, list![String::from("4")]);
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

    //let a: List<i32> = list![];
    //assert_eq!(a.head(), None);
}

#[test]
fn list_tail() {
    let a: List<i32> = list![1, 2, 3];
    assert_eq!(a.tail(), Rc::new(list![2, 3]));

    let a: List<i32> = list![1, 2];
    let b: List<i32> = list![1, 2, 3];
    assert!(a.tail() != b.tail());
}

#[test]
fn list_append() {
    let a: List<i32> = list![1];
    assert_eq!(a.append(list![2, 3]), list![1, 2, 3]);
}
