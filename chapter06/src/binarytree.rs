#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter01::interface::List;
use chapter02::arraydeque::Array as ArrayDeque;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree = Option<Rc<BTNode>>;

#[derive(Clone, Debug, Default)]
pub struct BTNode {
    left: RefCell<Option<Rc<BTNode>>>,
    right: RefCell<Option<Rc<BTNode>>>,
    parent: RefCell<Option<Weak<BTNode>>>,
}

#[derive(Clone, Debug, Default)]
pub struct BinaryTree {
    r: Option<Rc<BTNode>>,
}

impl BTNode {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl BinaryTree {
    pub fn new(r: Rc<BTNode>) -> Self {
        Self { r: Some(r) }
    }

    pub fn depth(&self, u: &Tree) -> usize {
        let mut u = u.clone();
        let mut d = 0;
        loop {
            match u {
                Some(n) if !Rc::ptr_eq(&n, self.r.as_ref().unwrap()) => {
                    u = n.parent.borrow().as_ref().and_then(|p| p.upgrade());
                    d += 1;
                }
                _ => break d,
            };
        }
    }

    fn size_u(u: &Tree) -> usize {
        match u {
            Some(n) => 1 + Self::size_u(&n.left.borrow()) + Self::size_u(&n.right.borrow()),
            None => 0,
        }
    }
    pub fn size(&self) -> usize {
        Self::size_u(&self.r)
    }

    pub fn size2(&self) -> usize {
        let mut c: usize = 0;
        let mut u = self.r.clone();
        let mut next: Option<Rc<BTNode>>;
        let mut prev: Option<Rc<BTNode>> = None;
        loop {
            match u {
                Some(ref n) => {
                    let parent = n.parent.borrow().as_ref().and_then(|p| p.upgrade());
                    let left = n.left.borrow().clone();
                    let right = n.right.borrow().clone();
                    match (prev, parent, left, right) {
                        (Some(p), Some(v), left, right) if Rc::ptr_eq(&p, &v) => {
                            c += 1;
                            next = left.or(right).or(Some(p));
                        }
                        (None, None, left, right) => {
                            c += 1;
                            next = left.or(right).or(None);
                        }
                        (Some(p), parent, Some(l), right) if Rc::ptr_eq(&p, &l) => {
                            next = right.or(parent);
                        }
                        (None, parent, None, right) => {
                            next = right.or(parent);
                        }
                        (_, parent, ..) => next = parent,
                    }
                }
                None => break c,
            }
            prev = u;
            u = next;
        }
    }

    fn height_u(u: &Tree) -> i32 {
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

    fn traverse_u(u: &Tree) {
        match u {
            Some(n) => {
                Self::traverse_u(&n.left.borrow());
                Self::traverse_u(&n.right.borrow());
            }
            None => (),
        }
    }

    pub fn traverse(&self) {
        Self::traverse_u(&self.r)
    }

    pub fn traverse2(&self) {
        let mut u = self.r.clone();
        let mut next: Option<Rc<BTNode>>;
        let mut prev: Option<Rc<BTNode>> = None;
        while let Some(ref n) = u {
            let parent = n.parent.borrow().as_ref().and_then(|p| p.upgrade());
            let left = n.left.borrow().clone();
            let right = n.right.borrow().clone();
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
            }
            prev = u;
            u = next;
        }
    }

    pub fn bf_traverse(&self) {
        let mut q: ArrayDeque<Rc<BTNode>> = ArrayDeque::new();
        if let Some(r) = self.r.clone() {
            q.add(q.size(), r)
        }
        while q.size() > 0 {
            if let Some(u) = q.remove(q.size() - 1) {
                if let Some(l) = u.left.borrow().as_ref() {
                    q.add(q.size(), l.clone())
                }
                if let Some(r) = u.right.borrow().as_ref() {
                    q.add(q.size(), r.clone())
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binarytree() {
        let r = Rc::new(BTNode::new());
        let n1 = Rc::new(BTNode::new());
        let n2 = Rc::new(BTNode::new());
        let n3 = Rc::new(BTNode::new());
        // r.left = n1, n1.parent = r
        // r.right = n2, n2.parent = r
        // n2.left = n3, n3.parent = n2
        r.left.borrow_mut().replace(Rc::clone(&n1));
        n1.parent.borrow_mut().replace(Rc::downgrade(&r));
        r.right.borrow_mut().replace(Rc::clone(&n2));
        n2.parent.borrow_mut().replace(Rc::downgrade(&r));
        n2.left.borrow_mut().replace(Rc::clone(&n3));
        n3.parent.borrow_mut().replace(Rc::downgrade(&n2));

        let binarytree = BinaryTree::new(Rc::clone(&r));
        assert_eq!(4, binarytree.size());
        assert_eq!(4, binarytree.size2());
        assert_eq!(2, binarytree.height());
        assert_eq!((), binarytree.traverse());
        assert_eq!((), binarytree.traverse2());
        assert_eq!((), binarytree.bf_traverse());
    }
}
