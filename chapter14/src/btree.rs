use crate::blockstore::BlockStore;
use chapter01::interface::SSet;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T: Clone + PartialOrd> {
    id: usize,
    keys: Box<[Option<T>]>,
    children: Box<[i32]>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct BTree<T: Clone + PartialOrd> {
    b: usize,  // the maximum number of children of a node (must be odd)
    B: usize,  // d div 2
    n: usize,  // number of elements stored in the tree
    ri: usize, // index of the root
    bs: BlockStore<Node<T>>,
}


impl<T: Clone + PartialOrd> Node<T> {
    fn new(t: &mut BTree<T>) -> Self {
        let b = t.b;
        let mut obj = Self {
            keys: vec![None; b].into_boxed_slice(),
            children: vec![-1i32; b + 1].into_boxed_slice(),
            id: 0,
        };
        obj.id = t.bs.place_block(obj.clone());
        obj
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
        if i >= n - 1 {
            self.keys[n-1] = Some(x);
        } else {
            self.keys[i..(n - 1)].rotate_right(1);
            let end = self.keys[i].replace(x);
            self.keys[n - 1] = end;
        }
        let n = self.children.len();
        if i + 1 >= n - 1 {
            self.children[n - 1] = ci;
        } else {
            self.children[(i + 1)..(n - 1)].rotate_right(1);
            self.children[n - 1] = ci;
            self.children.swap(i + 1, n - 1);
        }
        true
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        let n = self.keys.len();
        let y = self.keys.get_mut(i)?.take();
        self.keys[i..n].rotate_left(1);
        y
    }
    fn split(&mut self, t: &mut BTree<T>) -> Option<Node<T>> {
        let mut w = Self::new(t);
        let j = self.keys.len() / 2;
        for (i, key) in self.keys[j..].iter_mut().enumerate() {
            w.keys[i] = key.take();
        }
        for (i, chd) in self.children[(j + 1)..].iter_mut().enumerate() {
            w.children[i] = *chd;
            *chd = -1;
        }
        t.bs.write_block(self.id, self.clone());
        Some(w)
    }
}

impl<T: Clone + PartialOrd> BTree<T> {
    pub fn new(b: usize) -> Self {
        let mut tree = Self {
            b: b | 1,
            B: b / 2,
            bs: BlockStore::new(),
            ri: 0,
            n: 0
        };
        tree.ri = Node::<T>::new(&mut tree).id;
        tree
    }
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
    fn add_recursive(&mut self, mut x: T, ui: usize) -> Result<Option<Node<T>>, ()> {
        if let Some(mut u) = self.bs.read_block(ui) {
            let i = Self::find_it(&u.keys, &x);
            if i < 0 {
                return Err(())
            }
            if u.children[i as usize] < 0 {
                u.add(x, -1);
                self.bs.write_block(u.id, u.clone());
            } else {
                let w = self.add_recursive(x, u.children[i as usize] as usize)?;
                if let Some(mut w) = w {
                    x = w.remove(0).unwrap();
                    u.add(x, w.id as i32);
                    self.bs.write_block(w.id, w);
                    self.bs.write_block(u.id, u.clone());
                }
            }
            if u.is_full() {
                Ok(u.split(self))
            } else {
                Ok(None)
            }
        } else {
            Err(())
        }
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
       match self.add_recursive(x, self.ri) {
           Ok(w) => {
               if let Some(mut w) = w {
                   let mut newroot = Node::new(self);
                   let x = w.remove(0);
                   newroot.children[0] = self.ri as i32;
                   newroot.keys[0] = x;
                   newroot.children[1] = w.id as i32;
                   self.bs.write_block(w.id, w);
                   self.ri = newroot.id;
                   self.bs.write_block(self.ri, newroot);
               }
               self.n += 1;
               true
           },
           Err(()) => false,
       }
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


#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::SSet;
    use chapter09::redblacktree::RedBlackTree;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_btree() {
        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut btree= BTree::<i32>::new(11);

        for _ in 0..5 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                btree.add(x);
                assert_eq!(redblacktree.size(), btree.size());
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = btree.find(&x);
                assert_eq!(y1, y2);
            }
        }
    }
}