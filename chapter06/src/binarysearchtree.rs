use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree<T> = Option<Rc<BTNode<T>>>;

#[derive(Clone, Debug, Default)]
pub struct BTNode<T> {
    x: RefCell<T>,
    left: RefCell<Option<Rc<BTNode<T>>>>,
    right: RefCell<Option<Rc<BTNode<T>>>>,
    parent: RefCell<Option<Weak<BTNode<T>>>>,
}

#[derive(Clone, Debug, Default)]
pub struct BinarySearchTree<T> {
    n: usize,
    r: Option<Rc<BTNode<T>>>,
}

impl<T: Default> BTNode<T> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    pub fn findeq(&self, x: &T) -> Option<T> {
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
        let mut prev;
        let mut next;
        loop {
            prev = w.clone();
            match w {
                Some(ref u) if x < &*u.x.borrow() => next = u.left.borrow().clone(),
                Some(ref u) if x > &*u.x.borrow() => next = u.right.borrow().clone(),
                Some(ref u) if x == &*u.x.borrow() => break Some(u.clone()),
                _ => break prev,
            }
            w = next;
        }
    }
    fn add_child(&mut self, p: &Tree<T>, u: &Rc<BTNode<T>>) -> bool {
        todo!()
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
        let u = Rc::new(BTNode::<T>::new());
        *u.x.borrow_mut() = x;
        self.add_child(&p, &u)
    }
    fn remove(&mut self, _: &T) -> std::option::Option<T> {
        todo!()
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
