#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree<T> = Option<Rc<BSTNode<T>>>;

#[derive(Clone, Debug, Default)]
pub struct BSTNode<T> {
    x: RefCell<T>,
    left: RefCell<Option<Rc<BSTNode<T>>>>,
    right: RefCell<Option<Rc<BSTNode<T>>>>,
    parent: RefCell<Option<Weak<BSTNode<T>>>>,
}

#[derive(Clone, Debug, Default)]
pub struct BinarySearchTree<T: PartialOrd + Clone> {
    n: usize,
    r: Option<Rc<BSTNode<T>>>,
}

impl<T: PartialOrd + Clone> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        while let Some(r) = self.r.clone() {
            self.splice(r);
        }
    }
}

impl<T: Default> BSTNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            x: RefCell::new(x),
            ..Default::default()
        }
    }
}

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { n: 0, r: None }
    }
    fn height_u(u: &Tree<T>) -> i32 {
        match u {
            Some(n) => {
                1 + std::cmp::max(
                    Self::height_u(&n.left.borrow()),
                    Self::height_u(&n.right.borrow()),
                )
            }
            None => -1,
        }
    }

    pub fn height(&self) -> i32 {
        Self::height_u(&self.r)
    }

    pub fn find_eq(&self, x: &T) -> Option<T> {
        let mut w = self.r.clone();
        let mut next;
        loop {
            match w {
                Some(ref u) if x < &*u.x.borrow() => next = u.left.borrow().clone(),
                Some(ref u) if x > &*u.x.borrow() => next = u.right.borrow().clone(),
                Some(ref u) if x == &*u.x.borrow() => break Some(u.x.borrow().clone()),
                _ => break None,
            }
            w = next;
        }
    }
    fn find_last(&self, x: &T) -> Tree<T> {
        let mut w = self.r.clone();
        let mut prev = None;
        let mut next;
        loop {
            match w {
                Some(ref u) => {
                    prev = w.clone();
                    if x < &*u.x.borrow() {
                        next = u.left.borrow().clone();
                    } else if x > &*u.x.borrow() {
                        next = u.right.borrow().clone();
                    } else {
                        break Some(u.clone());
                    }
                }
                _ => break prev,
            }
            w = next;
        }
    }
    fn add_child(&mut self, p: &Tree<T>, u: Rc<BSTNode<T>>) -> bool {
        match p {
            Some(p) => {
                if *p.x.borrow() > *u.x.borrow() {
                    p.left.borrow_mut().replace(u.clone());
                } else if *p.x.borrow() < *u.x.borrow() {
                    p.right.borrow_mut().replace(u.clone());
                } else {
                    return false;
                }
                u.parent.borrow_mut().replace(Rc::downgrade(p));
            }
            None => self.r = Some(u),
        }
        self.n += 1;
        true
    }
    fn splice(&mut self, u: Rc<BSTNode<T>>) -> Option<T> {
        let s: Tree<T>;
        let mut p: Tree<T> = None;
        if u.left.borrow().is_some() {
            s = u.left.borrow_mut().take();
        } else {
            s = u.right.borrow_mut().take();
        }
        if let Some(r) = &self.r {
            if Rc::ptr_eq(&u, r) {
                self.r = s.clone();
                p = None;
            } else {
                p = u.parent.borrow_mut().take().and_then(|p| p.upgrade());
                if let Some(p) = p.as_ref() {
                    let left = p.left.borrow().clone();
                    match left {
                        Some(ref left) if Rc::ptr_eq(left, &u) => {
                            *p.left.borrow_mut() = s.clone();
                        }
                        _ => {
                            *p.right.borrow_mut() = s.clone();
                        }
                    }
                }
            }
        }
        match (s, p) {
            (Some(ref s), Some(ref p)) => {
                s.parent.borrow_mut().replace(Rc::downgrade(p));
            }
            (Some(ref s), None) => {
                s.parent.borrow_mut().take();
            }
            _ => (),
        }
        self.n -= 1;
        Some(Rc::try_unwrap(u).ok().unwrap().x.into_inner())
    }
    fn remove_u(&mut self, u: Rc<BSTNode<T>>) -> Option<T> {
        if u.left.borrow().is_none() || u.right.borrow().is_none() {
            self.splice(u)
        } else {
            let mut w = u.right.borrow().clone();
            loop {
                let mut next = None;
                if let Some(ref w) = w {
                    match *w.left.borrow() {
                        Some(ref left) => next = Some(left.clone()),
                        None => break,
                    }
                }
                w = next;
            }
            u.x.swap(&w.as_ref().unwrap().x);
            self.splice(w.unwrap())
        }
    }
}

impl<T> SSet<T> for BinarySearchTree<T>
where
    T: Ord + Clone + Default,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let p = self.find_last(&x);
        let u = Rc::new(BSTNode::<T>::new(x));
        self.add_child(&p, u)
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        match self.find_last(x) {
            Some(u) if &*u.x.borrow() == x => self.remove_u(u),
            _ => None,
        }
    }
    fn find(&self, x: &T) -> Option<T> {
        let mut w = self.r.clone();
        let mut z: Tree<T> = None;
        let mut next;
        loop {
            match w {
                Some(ref u) if x < &*u.x.borrow() => {
                    z = w.clone();
                    next = u.left.borrow().clone()
                }
                Some(ref u) if x > &*u.x.borrow() => next = u.right.borrow().clone(),
                Some(ref u) if x == &*u.x.borrow() => break Some(u.x.borrow().clone()),
                _ => {
                    break match z {
                        Some(z) => Some(z.x.borrow().clone()),
                        None => None,
                    }
                }
            }
            w = next;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::SSet;
    #[test]
    fn test_binarysearchtree() {
        let mut binarysearchtree = BinarySearchTree::<u32>::new();
        binarysearchtree.add(7);
        binarysearchtree.add(3);
        binarysearchtree.add(11);
        binarysearchtree.add(1);
        binarysearchtree.add(5);
        binarysearchtree.add(9);
        binarysearchtree.add(13);
        binarysearchtree.add(4);
        binarysearchtree.add(6);
        binarysearchtree.add(8);
        binarysearchtree.add(12);
        binarysearchtree.add(14);
        assert_eq!(false, binarysearchtree.add(8));
        assert_eq!(Some(6), binarysearchtree.remove(&6));
        assert_eq!(Some(9), binarysearchtree.remove(&9));
        assert_eq!(Some(11), binarysearchtree.remove(&11));
        assert_eq!(None, binarysearchtree.remove(&11));
        assert_eq!(Some(12), binarysearchtree.find(&12));
        assert_eq!(9, binarysearchtree.size());
        //println!("{:?}", binarysearchtree);

        // test large linked list for stack overflow.
        let mut bst = BinarySearchTree::<i32>::new();
        let num = 10000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
