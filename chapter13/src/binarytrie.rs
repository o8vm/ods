#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop)]
use crate::USizeV;
use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, Default)]
pub struct BTNode<T: USizeV + Default> {
    x: RefCell<T>,
    child: [RefCell<Option<Rc<BTNode<T>>>>; 2], // 0 = left, 1 = right
    jump: RefCell<Option<Rc<BTNode<T>>>>,
    parent: RefCell<Option<Weak<BTNode<T>>>>,
    prev: RefCell<Option<Weak<BTNode<T>>>>, // left
    next: RefCell<Option<Rc<BTNode<T>>>>,   // right
}

impl<T: USizeV + Default> BTNode<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct BinaryTrie<T: USizeV + Default + PartialOrd + Clone> {
    n: usize,
    r: Rc<BTNode<T>>,
    head: Option<Rc<BTNode<T>>>,   // dummy1
    tail: Option<Weak<BTNode<T>>>, // dummy2
}

impl<T: PartialOrd + Clone + Default + USizeV> Drop for BinaryTrie<T> {
    fn drop(&mut self) {
        while let Some(ref x) = self.head.as_ref().and_then(|s| {
            s.next
                .borrow()
                .as_ref()
                .filter(|n| n.next.borrow().is_some())
                .map(|n| n.x.borrow().clone())
        }) {
            self.remove(x);
        }
    }
}

impl<T: USizeV + Default + PartialOrd + Clone> BinaryTrie<T> {
    const W: usize = 64;
    pub fn new() -> Self {
        let r = Rc::new(BTNode::new());
        let dummy1: Rc<BTNode<T>> = Default::default();
        let dummy2: Rc<BTNode<T>> = Default::default();
        *dummy1.next.borrow_mut() = Some(dummy2.clone());
        *dummy2.prev.borrow_mut() = Some(Rc::downgrade(&dummy1));
        *r.jump.borrow_mut() = Some(dummy2.clone());
        Self {
            r,
            n: 0,
            head: Some(dummy1),
            tail: Some(Rc::downgrade(&dummy2)),
        }
    }
}

impl<T: USizeV + Default + PartialOrd + Clone> SSet<T> for BinaryTrie<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let mut c = 0;
        let ix = x.usize_value();
        let mut u = self.r.clone();

        // 1 - search for ix until falling out of the trie
        let mut i = 0;
        let mut next;
        for _ in 0..Self::W {
            c = (ix >> (Self::W - i - 1)) & 1;
            match *u.child[c].borrow() {
                Some(ref c) => next = c.clone(),
                None => break,
            }
            u = next;
            i += 1;
        }
        if i == Self::W {
            return false; // already contains x - abort
        }
        let pred = match c {
            0 => {
                let j = u.jump.borrow_mut().take();
                match j {
                    Some(ref j) => j.prev.borrow().as_ref().and_then(|p| p.upgrade()),
                    None => None,
                }
            }
            _ => u.jump.borrow_mut().take(), // right
        };
        // 2 - add path to ix
        while i < Self::W {
            c = (ix >> (Self::W - i - 1)) & 1;
            let n = Rc::new(BTNode::new());
            n.parent.borrow_mut().replace(Rc::downgrade(&u));
            u.child[c].borrow_mut().replace(n);
            let uc = u.child[c].borrow().clone().unwrap();
            u = uc;
            i += 1;
        }
        *u.x.borrow_mut() = x;

        // 3 - add u to linked list
        *u.prev.borrow_mut() = pred.as_ref().map(|p| Rc::downgrade(&p));
        *u.next.borrow_mut() = pred.as_ref().and_then(|p| p.next.borrow().clone());
        u.prev
            .borrow()
            .as_ref()
            .map(|p| p.upgrade().map(|p| p.next.borrow_mut().replace(u.clone())));
        u.next
            .borrow()
            .as_ref()
            .map(|n| n.prev.borrow_mut().replace(Rc::downgrade(&u)));

        // 4 - walk back up, updating jump pointers
        let mut v = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
        while let Some(vi) = v {
            if (vi.child[0].borrow().is_none()
                && (vi.jump.borrow().is_none()
                    || vi
                        .jump
                        .borrow()
                        .as_ref()
                        .filter(|j| (*j.x.borrow()).usize_value() > ix)
                        .is_some()))
                || (vi.child[1].borrow().is_none()
                    && (vi.jump.borrow().is_none()
                        || vi
                            .jump
                            .borrow()
                            .as_ref()
                            .filter(|j| (*j.x.borrow()).usize_value() < ix)
                            .is_some()))
            {
                vi.jump.borrow_mut().replace(u.clone());
            }
            v = vi.parent.borrow().as_ref().and_then(|p| p.upgrade());
        }
        self.n += 1;
        true
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        let mut c;
        let ix = x.usize_value();
        let mut u = self.r.clone();

        // 1 - find leaf, u, containing x
        let mut i = 0;
        let mut next;
        for _ in 0..Self::W {
            c = (ix >> (Self::W - i - 1)) & 1;
            match *u.child[c].borrow() {
                Some(ref c) => next = c.clone(),
                None => return None,
            }
            u = next;
            i += 1;
        }

        // 2 - remove u from linked list
        let next = u.next.borrow_mut().take();
        let prev = u.prev.borrow_mut().take();
        if let Some(n) = next.as_ref() {
            *n.prev.borrow_mut() = prev.clone();
        }
        if let Some(p) = prev.as_ref() {
            *p.upgrade().unwrap().next.borrow_mut() = next.clone();
        }
        let mut v = u.clone();

        // 3 - delete nodes on path to u
        for i in (0..Self::W).rev() {
            c = (ix >> (Self::W - i - 1)) & 1;
            let vp = v
                .parent
                .borrow()
                .as_ref()
                .and_then(|p| p.upgrade())
                .unwrap();
            v = vp;
            v.child[c].borrow_mut().take();
            if v.child[1 - c].borrow().is_some() {
                break;
            }
        }

        // 4 - update jump pointers
        c = if v.child[0].borrow().is_none() { 1 } else { 0 };
        *v.jump.borrow_mut() = if c == 0 {
            prev.as_ref().and_then(|p| p.upgrade())
        } else {
            next.clone()
        };
        let mut v = v.parent.borrow().as_ref().and_then(|p| p.upgrade());
        while let Some(vi) = v {
            if vi
                .jump
                .borrow()
                .as_ref()
                .filter(|j| Rc::ptr_eq(j, &u))
                .is_some()
            {
                let c = if vi.child[0].borrow().is_none() { 1 } else { 0 };
                *vi.jump.borrow_mut() = if c == 0 {
                    prev.as_ref().and_then(|p| p.upgrade())
                } else {
                    next.clone()
                };
            }
            v = vi.parent.borrow().as_ref().and_then(|p| p.upgrade());
        }
        self.n -= 1;
        Some(Rc::try_unwrap(u).ok().unwrap().x.into_inner())
    }
    fn find(&self, x: &T) -> Option<T> {
        let mut i = 0;
        let mut c = 0;
        let ix = x.usize_value();
        let mut u = self.r.clone();
        let mut next;
        for _ in 0..Self::W {
            c = (ix >> (Self::W - i - 1)) & 1;
            match *u.child[c].borrow() {
                Some(ref c) => next = c.clone(),
                None => break,
            }
            u = next;
            i += 1;
        }
        if i == Self::W {
            return Some(u.x.borrow().clone());
        }
        let n = if c == 0 {
            u.jump.borrow().clone()
        } else {
            let j = u.jump.borrow().clone();
            match j {
                Some(ref j) => j.next.borrow().clone(),
                None => None,
            }
        };
        match n {
            Some(ref n) if n.next.borrow().is_none() => None,
            Some(ref n) if n.prev.borrow().is_none() => None,
            _ => n.as_ref().map(|u| u.x.borrow().clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::SSet;
    use chapter09::redblacktree::RedBlackTree;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_binarytrie() {
        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut binarytrie = BinaryTrie::new();

        for _ in 0..5 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                binarytrie.add(x);
                assert_eq!(redblacktree.size(), binarytrie.size());
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = binarytrie.find(&x);
                assert_eq!(y1, y2);
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let b1 = redblacktree.remove(&x);
                let b2 = binarytrie.remove(&x);
                assert_eq!(b1, b2);
            }
            assert_eq!(redblacktree.size(), binarytrie.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = binarytrie.find(&x);
                assert_eq!(y1, y2);
            }
        }

        // test large linked list for stack overflow.
        let mut bst = BinaryTrie::<i32>::new();
        let num = 100000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
