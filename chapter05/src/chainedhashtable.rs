#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use super::hashcode;
use chapter01::interface::{List, USet};
use chapter02::arraystack::Array as ArrayStack;
use std::hash::Hash;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChainedHashTable<T> {
    t: Box<[ArrayStack<T>]>,
    n: usize,
    d: usize,
    z: usize,
}

impl<T> ChainedHashTable<T>
where
    T: PartialEq + Clone + Hash,
{
    const W: usize = std::mem::size_of::<usize>() * 8;
    pub fn new() -> Self {
        Self {
            t: Self::allocate_in_heap(2),
            n: 0,
            d: 1,
            z: rand::random::<usize>() | 1,
        }
    }
    fn allocate_in_heap(size: usize) -> Box<[ArrayStack<T>]> {
        std::iter::repeat_with(|| ArrayStack::new())
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
    fn resize(&mut self) {
        self.d = 1;
        while 1 << self.d <= self.n {
            self.d += 1
        }
        self.n = 0;
        let new_t = Self::allocate_in_heap(1 << self.d);
        let old_t = std::mem::replace(&mut self.t, new_t);
        for elem in old_t.into_vec().iter_mut() {
            let len = elem.size();
            for _j in 0..len {
                self.add(elem.remove(0).unwrap());
            }
        }
    }
    fn hash(&self, x: &T) -> usize {
        (((self.z as u128 * hashcode(x) as u128) % ((1 as u128) << Self::W as u128))
            >> (Self::W - self.d) as u128) as usize
    }
}

impl<T> USet<T> for ChainedHashTable<T>
where
    T: PartialEq + Clone + Hash,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        if self.find(&x).is_some() {
            return false;
        }
        if self.n + 1 > self.t.len() {
            self.resize();
        }
        if let Some(t) = self.t.get_mut(self.hash(&x)) {
            t.add(t.size(), x)
        }
        self.n += 1;
        true
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        let j = self.hash(x);
        for i in 0..self.t.get(j)?.size() {
            if Some(x) == self.t.get(j).and_then(|t| t.get(i)).as_ref() {
                let y = self.t.get_mut(j)?.remove(i);
                self.n -= 1;
                if 3 * self.n < self.t.len() {
                    self.resize()
                }
                return y;
            }
        }
        None
    }
    fn find(&self, x: &T) -> Option<T> {
        let j = self.hash(x);
        for i in 0..self.t.get(j)?.size() {
            if Some(x) == self.t.get(j).and_then(|t| t.get(i)).as_ref() {
                return self.t.get(j)?.get(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::ChainedHashTable;
    use chapter01::interface::USet;

    #[test]
    fn test_chainedhashtable() {
        let mut chainedhashtable = ChainedHashTable::<char>::new();
        chainedhashtable.add('a');
        chainedhashtable.add('b');
        chainedhashtable.add('c');
        chainedhashtable.add('d');
        chainedhashtable.add('e');
        chainedhashtable.add('f');
        chainedhashtable.add('g');
        chainedhashtable.add('h');
        chainedhashtable.add('i');
        chainedhashtable.add('j');
        chainedhashtable.add('k');
        chainedhashtable.add('l');
        chainedhashtable.add('m');
        chainedhashtable.add('x');
        for elem in "abcdefghijklmx".chars() {
            assert_eq!(Some(elem), chainedhashtable.find(&elem));
        }
        assert_eq!(chainedhashtable.remove(&'x'), Some('x'));
        assert_eq!(chainedhashtable.remove(&'a'), Some('a'));
        assert_eq!(chainedhashtable.remove(&'b'), Some('b'));
        assert_eq!(chainedhashtable.remove(&'c'), Some('c'));
        assert_eq!(chainedhashtable.remove(&'d'), Some('d'));
        assert_eq!(chainedhashtable.remove(&'e'), Some('e'));
        assert_eq!(chainedhashtable.remove(&'f'), Some('f'));
        assert_eq!(chainedhashtable.remove(&'g'), Some('g'));
        assert_eq!(chainedhashtable.remove(&'h'), Some('h'));
        assert_eq!(chainedhashtable.remove(&'i'), Some('i'));
        assert_eq!(chainedhashtable.remove(&'x'), None);
    }
}
