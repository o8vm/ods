#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use super::{byte_chunks_64, Tabulation};
use chapter01::interface::USet;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::hash::Hash;

lazy_static! {
    pub static ref TAB: [[u64; 256]; 8] = {
        let mut array = [[0; 256]; 8];
        for item in &mut array {
            thread_rng().fill(item);
        }
        array
    };
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Copy)]
enum Elem<T> {
    Val(T),
    Null,
    Del,
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinearHashTable<T> {
    t: Box<[Elem<T>]>,
    n: usize,
    q: usize,
    d: u32,
}

impl<T> Default for Elem<T> {
    fn default() -> Self {
        Elem::Null
    }
}
impl<T: Hash> Tabulation for T {}
impl<T> LinearHashTable<T>
where
    T: PartialEq + Clone + Hash,
{
    const W: u32 = (std::mem::size_of::<usize>() * 8) as u32;
    pub fn new() -> Self {
        Self {
            t: Self::allocate_in_heap(2),
            n: 0,
            q: 1,
            d: 1,
        }
    }
    fn allocate_in_heap(size: usize) -> Box<[Elem<T>]> {
        std::iter::repeat_with(|| Default::default())
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
    fn hash(&self, x: &T) -> usize {
        // u64 tabulation hashing

        let mut v = 0u64;
        let h = x.hashcode();
        let chunks = byte_chunks_64(h as u64);
        for (i, c) in chunks.iter().enumerate() {
            v ^= TAB[i][*c as usize];
        }
        v = v.overflowing_shr(Self::W - self.d).0;
        v as usize
    }
    fn resize(&mut self) {
        self.d = 1;
        while (1 << self.d) < 3 * self.n {
            self.d += 1;
        }
        let new_t = Self::allocate_in_heap(1 << self.d);
        let old_t = std::mem::replace(&mut self.t, new_t);
        for oelem in old_t.into_vec().into_iter() {
            match oelem {
                Elem::Val(x) => {
                    let mut i = self.hash(&x);
                    loop {
                        match self.t.get(i) {
                            Some(nelem) if nelem != &Elem::Null => {
                                i = if i == self.t.len() - 1 { 0 } else { i + 1 }
                            }
                            _ => break,
                        }
                    }
                    if let Some(elem) = self.t.get_mut(i) {
                        *elem = Elem::Val(x);
                    }
                }
                _ => continue,
            }
        }
    }
}

impl<T> USet<T> for LinearHashTable<T>
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
        if 2 * (self.q + 1) > self.t.len() {
            self.resize();
        }
        let mut i = self.hash(&x);
        loop {
            match self.t.get(i) {
                Some(elem) => match elem {
                    Elem::Val(_y) => i = if i == self.t.len() - 1 { 0 } else { i + 1 },
                    _ => break,
                },
                None => return false,
            }
        }
        if self.t.get(i).unwrap() == &Elem::Null {
            self.q += 1
        }
        self.n += 1;
        if let Some(elem) = self.t.get_mut(i) {
            *elem = Elem::Val(x)
        }
        true
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        let mut i = self.hash(x);
        loop {
            match self.t.get_mut(i) {
                Some(elem) if elem != &Elem::Null => match elem {
                    Elem::Val(y) if y == x => {
                        let y = std::mem::replace(elem, Elem::Del);
                        self.n -= 1;
                        if 8 * self.n < self.t.len() {
                            self.resize()
                        }
                        break match y {
                            Elem::Val(y) => Some(y),
                            _ => None,
                        };
                    }
                    _ => i = if i == self.t.len() - 1 { 0 } else { i + 1 },
                },
                _ => break None,
            }
        }
    }
    fn find(&self, x: &T) -> Option<T> {
        let mut i = self.hash(x);
        loop {
            match self.t.get(i) {
                Some(elem) if elem != &Elem::Null => match elem {
                    Elem::Val(y) if y == x => break Some(y.clone()),
                    _ => i = if i == self.t.len() - 1 { 0 } else { i + 1 },
                },
                _ => break None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinearHashTable;
    use chapter01::interface::USet;
    #[test]
    fn test_linearhashtable() {
        let mut linearhashtable = LinearHashTable::<char>::new();
        linearhashtable.add('a');
        linearhashtable.add('b');
        linearhashtable.add('c');
        linearhashtable.add('d');
        linearhashtable.add('e');
        linearhashtable.add('x');
        assert_eq!(false, linearhashtable.add('x'));
        for elem in "abcdex".chars() {
            assert_eq!(linearhashtable.find(&elem), Some(elem));
        }
        assert_eq!(linearhashtable.remove(&'x'), Some('x'));
        assert_eq!(linearhashtable.remove(&'x'), None);
        assert_eq!(linearhashtable.remove(&'a'), Some('a'));
        assert_eq!(linearhashtable.remove(&'b'), Some('b'));
        assert_eq!(linearhashtable.remove(&'c'), Some('c'));
        assert_eq!(linearhashtable.remove(&'e'), Some('e'));
        assert_eq!(linearhashtable.remove(&'a'), None);
        println!("{:?}", linearhashtable);
    }
}
