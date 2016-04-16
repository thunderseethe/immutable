use std::cmp::{ PartialEq, Eq, Ord, PartialOrd, Ordering };
use std::iter::{Iterator, IntoIterator, FromIterator};
//use std::hash::{ Hash, Hasher };

use tree::binary_tree::{BinaryTree, Iter};

#[derive(Clone)]
pub struct Entry<K: Eq + Ord, V> {
    key: K,
    val: V,
}

impl<K: Ord + Eq, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
    fn ne(&self, other: &Self) -> bool { self.key != other.key }
}
impl<K: Ord + Eq, V> PartialEq<K> for Entry<K, V> {
    fn eq(&self, other: &K) -> bool { self.key == *other }
    fn ne(&self, other: &K) -> bool { self.key != *other }
}
impl<K: Ord + Eq, V> Eq for Entry<K, V> {}

impl<K: Ord + Eq, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.key.partial_cmp(&other.key) }
    fn lt(&self, other: &Self) -> bool { self.key < other.key }
    fn le(&self, other: &Self) -> bool { self.key <= other.key }
    fn gt(&self, other: &Self) -> bool { self.key > other.key }
    fn ge(&self, other: &Self) -> bool { self.key >= other.key }
}
impl<K: Ord + Eq, V> PartialOrd<K> for Entry<K, V> {
    fn partial_cmp(&self, other: &K) -> Option<Ordering> { self.key.partial_cmp(other) }
    fn lt(&self, other: &K) -> bool { self.key < *other }
    fn le(&self, other: &K) -> bool { self.key <= *other }
    fn gt(&self, other: &K) -> bool { self.key > *other }
    fn ge(&self, other: &K) -> bool { self.key >= *other }
}

impl<K: Eq + Ord, V> Entry<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Entry {
            key: key,
            val: val
        }
    }
}
// impl<K: Ord + Eq, V> PartialOrd<Entry<K, V>> for K {
//     fn partial_cmp(&self, other: &Entry<K, V>) -> Option<Ordering> { *self.partial_cmp(other.key) }
//     fn lt(&self, other: &Entry<K, V>) -> bool { *self < other.key }
//     fn le(&self, other: &Entry<K, V>) -> bool { *self <= other.key }
//     fn gt(&self, other: &Entry<K, V>) -> bool { *self > other.key }
//     fn ge(&self, other: &Entry<K, V>) -> bool { *self >= other.key }
// }

impl<K: Ord + Eq, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> Ordering { self.key.cmp(&other.key) }
}
/*impl<K: Ord + Eq, V: Eq> Ord<K> for Entry<K, V> {
    fn cmp(&self, other: &K) -> Ordering { self.key.cmp(other) }
}*/

pub struct Map<K: Clone + Ord + Eq, V: Clone> {
    tree: BinaryTree<Entry<K, V>>,
}

impl<K: Clone + Ord + Eq, V: Clone> Map<K, V> {
    pub fn new() -> Self {
        Map {
            tree: BinaryTree::empty()
        }
    }

    pub fn get(&self, key: K) -> Option<Entry<K, V>> {
        self.tree.get(key)
    }

    pub fn put(self, key: K, val: V) -> Self {
        Map { tree: self.tree.insert(Entry::new(key, val)) }
    }
}

impl<K: Clone + Ord + Eq, V: Clone> IntoIterator for Map<K, V> {
    type Item = Entry<K, V>;
    type IntoIter = Iter<Entry<K, V>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.tree.into_iter()
    }
}

impl<K: Clone + Ord + Eq, V: Clone> FromIterator<Entry<K, V>> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item=Entry<K, V>>>(iterator: I) -> Self {
        iterator
            .into_iter()
            .fold(Map::new(), | map, Entry{key, val} | map.put(key, val))
    }
}
impl<K: Clone + Ord + Eq, V: Clone> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item=(K, V)>>(iterator: I) -> Self {
        iterator
            .into_iter()
            .fold(Map::new(), | map, (k, v) | map.put(k, v))
    }
}


#[test]
fn map_macro() {
    assert!(true);
}
