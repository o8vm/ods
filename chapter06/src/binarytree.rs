use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree = Option<Rc<RefCell<BTNode>>>;

#[derive(Clone, Debug, Default)]
pub struct BTNode {
    left: Tree,
    right: Tree,
    parent: Option<Weak<RefCell<BTNode>>>,
}

pub struct BinaryTree {
    r: Tree,
}

impl BTNode {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl BinaryTree {
    pub fn depth(&self, u: &Tree) -> usize {
        let mut u = u.clone();
        let mut d = 0;
        loop {
            match u {
                Some(n) if !Rc::ptr_eq(&n, self.r.as_ref().unwrap()) => {
                    u = n.borrow().parent.as_ref().and_then(|p| p.upgrade());
                    d += 1;
                }
                _ => break d,
            }
        }
    }

    pub fn size(u: &Tree) -> usize {
        match u {
            Some(n) => 1 + Self::size(&n.borrow().left) + Self::size(&n.borrow().right),
            None => 0,
        }
    }

    pub fn size2() -> usize {
        todo!()
    }

    pub fn height(u: &Tree) -> i32 {
        match u {
            Some(n) => {
                1 + std::cmp::max(
                    Self::height(&n.borrow().left),
                    Self::height(&n.borrow().right),
                )
            }
            None => -1,
        }
    }

    pub fn traverse(u: &Tree) {
        match u {
            Some(n) => {
                Self::traverse(&n.borrow().left);
                Self::traverse(&n.borrow().right);
            }
            None => (),
        }
    }

    pub fn traverse2(&self) {
        let mut u = self.r.clone();
        let mut next: Tree;
        let mut prev: Tree = None;
        loop {
            match u {
                Some(ref n) => {
                    let parent = n.borrow().parent.as_ref().and_then(|n| n.upgrade());
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();
                    match (prev, parent, left, right) {
                        (Some(p), Some(v), left, right) if Rc::ptr_eq(&p, &v) => {
                            next = left.or(right).or(Some(p));
                        }
                        (None, None, left, right) => {
                            next = left.or(right).or(None);
                        }
                        (Some(p), parent, Some(l), right) if Rc::ptr_eq(&p, &l) => {
                            next = right.or(parent);
                        }
                        (None, parent, None, right) => {
                            next = right.or(parent);
                        }
                        (_, parent, ..) => next = parent,
                    };
                }
                None => break,
            };
            prev = u;
            u = next;
        }
    }
}
