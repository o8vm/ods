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
        match p {
            Some(p) => {
                if *p.x.borrow() < *u.x.borrow() {
                    p.left.borrow_mut().replace(u.clone());
                } else if *p.x.borrow() > *u.x.borrow() {
                    p.right.borrow_mut().replace(u.clone());
                } else {
                    return false;
                }
                u.parent.borrow_mut().replace(Rc::downgrade(p));
            }
            None => self.r = p.clone(),
        }
        self.n += 1;
        true
    }
    fn splice(&mut self, u: Rc<BTNode<T>>) {
        let s: Tree<T>;
        let mut p: Tree<T> = None;
        match *u.left.borrow() {
            Some(ref l) => s = Some(l.clone()),
            None => s = u.right.borrow().clone(),
        }
        if let Some(r) = &self.r {
            if Rc::ptr_eq(&u, r) {
                self.r = s.clone();
                p = None;
            } else {
                p = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
                p.as_ref().map(|p| {
                    if let Some(ref left) = *p.left.borrow() {
                        if Rc::ptr_eq(left, &u) {
                            *p.left.borrow_mut() = s.clone();
                        } else {
                            *p.right.borrow_mut() = s.clone();
                        }
                    }
                });
            }
        }
        if let (Some(ref s), Some(ref p)) = (s, p) {
            s.parent.borrow_mut().replace(Rc::downgrade(&p));
        }
        self.n -= 1;
    }
    fn remove_u(&mut self, u: Rc<BTNode<T>>) {
        if u.left.borrow().is_none() || u.right.borrow().is_none() {
            self.splice(u);
        } else {
            let mut w = u.right.borrow().clone();
            let mut next = None;
            loop {
                if let Some(ref w) = w {
                    match *w.left.borrow() {
                        Some(ref left) => next = Some(left.clone()),
                        None => break,
                    }
                }
                w = next.clone();
            }
            *u.x.borrow_mut() = w.as_ref().unwrap().x.borrow().clone();
            self.splice(w.unwrap());
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
