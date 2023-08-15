use std::collections::{BinaryHeap, BTreeMap, HashSet, LinkedList};
use std::hash::Hash;

pub fn add_vec(xs: &[u64]) -> Vec<u64> {
    let mut vec = Vec::new();
    for x in xs {
        vec.push(*x);
    }
    vec
}

pub fn add_vec_capacity(xs: &[u64]) -> Vec<u64> {
    let mut vec = Vec::with_capacity(xs.len());
    for x in xs {
        vec.push(*x);
    }
    vec
}

pub fn add_hashset<T>(xs: &[T]) -> HashSet<T> where T: Copy + Eq + Hash {
    let mut set = HashSet::new();
    for x in xs {
        set.insert(*x);
    }
    set
}

pub fn add_btree(xs: &[u64]) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    for x in xs {
        map.insert(*x, *x);
    }
    map
}

pub fn add_heap(xs: &[u64]) -> BinaryHeap<u64> {
    let mut heap = BinaryHeap::new();
    for x in xs {
        heap.push(*x);
    }
    heap
}

pub fn add_linked_list(xs: &[u64]) -> LinkedList<u64> {
    let mut list = LinkedList::new();
    for x in xs {
        list.push_back(*x);
    }
    list
}