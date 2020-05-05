use super::hashcode;
use chapter01::interface::USet;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Copy)]
enum Elem<T> {
    Val(T),
    Null,
    Del,
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinerHashTable<T> {
    t: Box<[Elem<T>]>,
    n: usize,
    q: usize,
    d: usize,
}

impl<T> Default for Elem<T> {
    fn default() -> Self {
        Elem::Null
    }
}

impl<T> LinerHashTable<T>
where
    T: Eq + Clone + Hash,
{
    fn allocate_in_heap<'a>(size: usize) -> Box<[Elem<T>]> {
        std::iter::repeat_with(|| Default::default())
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
    fn hash(&self, x: &T) -> usize {
        todo!()
    }
    fn resize(&mut self) {
        self.d = 1;
        while (1 << self.d) < 3 * self.n {
            self.d += 1;
        }
        let new_t = Self::allocate_in_heap(1 << self.d);
        let old_t = std::mem::replace(&mut self.t, new_t);
        for elem in old_t.into_vec().into_iter() {
            match elem {
                Elem::Val(y) => todo!(),
                _ => continue,
            }
        }
        todo!()
    }
}

impl<T> USet<T> for LinerHashTable<T>
where
    T: Eq + Clone + Hash,
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
