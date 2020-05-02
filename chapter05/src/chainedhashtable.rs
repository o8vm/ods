use chapter01::interface::{List, USet};
use chapter02::arraystack::Array as ArrayStack;

pub struct ChainedHashTable<T> {
    t: Box<[ArrayStack<T>]>,
    n: usize,
    d: usize,
    z: usize,
}

impl<T> ChainedHashTable<T> {
    const W: usize = std::mem::size_of::<usize>() * 8;
    fn resize(&mut self) {
        todo!()
    }
    fn hash(&self, x: &T) -> usize {
        (self.z * Self::hashcode(x)) >> (Self::W - self.d)
    }
    fn hashcode(x: &T) -> usize {
        todo!()
    }
}

impl<T: Eq + Clone> USet<T> for ChainedHashTable<T> {
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
        self.t.get_mut(self.hash(&x)).map(|t| t.add(t.size(), x));
        self.n += 1;
        true
    }
    fn remove(&mut self, x: &T) -> std::option::Option<T> {
        let j = self.hash(x);
        for i in 0..self.t.get(j)?.size() {
            if Some(x) == self.t.get(j).and_then(|t| t.get(i)).as_ref() {
                self.n -= 1;
                return self.t.get_mut(j)?.remove(i);
            }
        }
        None
    }
    fn find(&self, x: &T) -> std::option::Option<T> {
        todo!()
    }
}
