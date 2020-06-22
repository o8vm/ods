use crate::blockstore::BlockStore;
use chapter01::interface::SSet;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node<T: Clone + PartialOrd> {
    t: BTree<T>,
    id: usize,
    keys: Box<[Option<T>]>,
    children: Box<[i32]>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BTree<T: Clone + PartialOrd> {
    b: usize,  // the maximum number of children of a node (must be odd)
    B: usize,  // d div 2
    n: usize,  // number of elements stored in the tree
    ri: usize, // index of the root
    bs: BlockStore<Node<T>>,
}

impl<T: Clone + PartialOrd> Node<T> {
    fn new(t: Btree<T>) -> Self {
        Self {
            t,
            keys: 
        }
    }
    fn is_leaf(&self) -> bool {
        self.children[0] < 0
    }
    fn is_full(&self) -> bool {
        self.keys[self.keys.len() - 1].is_some()
    }
    fn size(&self) -> usize {
        let mut lo = 0;
        let mut hi = self.keys.len();
        while hi != lo {
            let m = (hi + lo) / 2;
            if self.keys[m].is_some() {
                hi = m;
            } else {
                lo = m + 1;
            }
        }
        lo
    }
    fn add(&mut self, x: T, ci: i32) -> bool {
        let i = BTree::<T>::find_it(&self.keys, &x);
        if i < 0 {
            return false;
        }
        let i = i as usize;
        let n = self.keys.len();
        self.keys[i..(n - 1)].rotate_right(1);
        let end = self.keys[i].replace(x);
        self.keys[n - 1] = end;
        let n = self.children.len();
        self.children[(i + 1)..(n - 1)].rotate_right(1);
        self.children[n - 1] = ci;
        self.children.swap(i + 1, n - 1);
        true
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        let n = self.keys.len();
        let y = self.keys.get_mut(i)?.take();
        self.keys[i..n].rotate_left(1);
        y
    }
    fn split(&mut self) -> Option<Node<T>> {
        todo!()
    }
}

impl<T: Clone + PartialOrd> BTree<T> {
    fn find_it(a: &[Option<T>], x: &T) -> i32 {
        let mut lo = 0;
        let mut hi = a.len();
        while hi != lo {
            let m = (hi + lo) / 2;
            match &a[m] {
                None => hi = m,
                Some(v) if x < v => hi = m,
                Some(v) if x > v => lo = m + 1,
                _ => return -(m as i32) - 1,
            }
        }
        lo as i32
    }
}

impl<T> SSet<T> for BTree<T>
where
    T: Clone + PartialOrd,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        todo!()
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        todo!()
    }
    fn find(&self, x: &T) -> Option<T> {
        let mut z = None;
        let mut ui = self.ri as i32;
        while ui >= 0 {
            let u = self.bs.read_block(ui as usize)?;
            let i = Self::find_it(&u.keys, &x);
            if i < 0 {
                return u.keys[(-(i + 1)) as usize].clone();
            }
            if u.keys[i as usize].is_some() {
                z = u.keys[i as usize].clone()
            }
            ui = u.children[i as usize];
        }
        z
    }
}
