#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter01::interface::Queue;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree<T> = Option<Rc<MHNode<T>>>;

#[derive(Clone, Debug, Default)]
pub struct MHNode<T> {
    x: RefCell<T>,
    left: RefCell<Option<Rc<MHNode<T>>>>,
    right: RefCell<Option<Rc<MHNode<T>>>>,
    parent: RefCell<Option<Weak<MHNode<T>>>>,
}

#[derive(Clone, Debug, Default)]
pub struct MeldableHeap<T: PartialOrd + Clone> {
    n: usize,
    r: Option<Rc<MHNode<T>>>,
}

impl<T: PartialOrd + Clone> Drop for MeldableHeap<T> {
    fn drop(&mut self) {
        while let Some(r) = self.r.clone() {
            self.splice(r);
        }
    }
}

impl<T: Default> MHNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            x: RefCell::new(x),
            ..Default::default()
        }
    }
}

impl<T: PartialOrd + Clone> MeldableHeap<T> {
    pub fn new() -> Self {
        Self { n: 0, r: None }
    }
    fn merge(h1: Tree<T>, h2: Tree<T>) -> Tree<T> {
        match (h1, h2) {
            (None, h2) => h2,
            (h1, None) => h1,
            (Some(h1), Some(h2)) if *h1.x.borrow() > *h2.x.borrow() => {
                Self::merge(Some(h2), Some(h1))
            }
            (Some(h1), Some(h2)) => {
                if rand::random::<bool>() {
                    let u = Self::merge(h1.left.borrow().clone(), Some(h2));
                    *h1.left.borrow_mut() = u;
                    if let Some(left) = &*h1.left.borrow() {
                        left.parent.borrow_mut().replace(Rc::downgrade(&h1));
                    }
                    Some(h1)
                } else {
                    let u = Self::merge(h1.right.borrow().clone(), Some(h2));
                    *h1.right.borrow_mut() = u;
                    if let Some(right) = &*h1.right.borrow() {
                        right.parent.borrow_mut().replace(Rc::downgrade(&h1));
                    }
                    Some(h1)
                }
            }
        }
    }
    pub fn find_min(&self) -> Option<T> {
        self.r.as_ref().map(|r| r.x.borrow().clone())
    }
    fn splice(&mut self, u: Rc<MHNode<T>>) -> Option<T> {
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
}

impl<T> Queue<T> for MeldableHeap<T>
where
    T: PartialOrd + Clone + Default,
{
    fn add(&mut self, x: T) {
        let u = Rc::new(MHNode::new(x));
        self.r = Self::merge(Some(u), self.r.clone());
        self.r.as_ref().and_then(|r| r.parent.borrow_mut().take());
        self.n += 1;
    }
    fn remove(&mut self) -> Option<T> {
        let u = self.r.take();
        self.r = Self::merge(
            u.as_ref().and_then(|r| r.left.borrow_mut().take()),
            u.as_ref().and_then(|r| r.right.borrow_mut().take()),
        );
        self.r.as_ref().and_then(|r| r.parent.borrow_mut().take());
        if self.n != 0 {
            self.n -= 1;
        }
        u.map(|u| Rc::try_unwrap(u).ok().unwrap().x.into_inner())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::Queue;
    #[test]
    fn test_meldableheap() {
        let mut meldableheap = MeldableHeap::<usize>::new();
        meldableheap.add(7);
        meldableheap.add(8);
        meldableheap.add(9);
        meldableheap.add(26);
        meldableheap.add(4);
        assert_eq!(meldableheap.n, 5);
        assert_eq!(meldableheap.remove(), Some(4));
        assert_eq!(meldableheap.n, 4);
        assert_eq!(meldableheap.remove(), Some(7));
        assert_eq!(meldableheap.remove(), Some(8));
        assert_eq!(meldableheap.remove(), Some(9));
        assert_eq!(meldableheap.remove(), Some(26));
        assert_eq!(meldableheap.n, 0);
        assert_eq!(meldableheap.remove(), None);
        println!("{:?}", meldableheap);

        // test large linked list for stack overflow.
        let mut bst = MeldableHeap::<i32>::new();
        let num = 100000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
