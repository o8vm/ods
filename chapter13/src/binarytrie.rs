use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree = Option<Rc<BTNode>>;

#[derive(Clone, Debug, Default)]
pub struct BTNode {
    x: RefCell<i32>,
    child: [RefCell<Option<Rc<BTNode>>>; 2], // 0 = left, 1 = right
    jump: RefCell<Option<Rc<BTNode>>>,
    parent: RefCell<Option<Weak<BTNode>>>,
    prev: RefCell<Option<Weak<BTNode>>>,
    next: RefCell<Option<Rc<BTNode>>>,
}

#[derive(Clone, Debug, Default)]
pub struct BinaryTrie {
    n: usize,
    r: Option<Rc<BTNode>>,
    head: Option<Rc<BTNode>>,   // dummy1
    tail: Option<Weak<BTNode>>, // dummy2
}

impl BinaryTrie {
    const W: usize = 32;
}

impl SSet<i32> for BinaryTrie {
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, _: i32) -> bool {
        todo!()
    }
    fn remove(&mut self, _: &i32) -> Option<i32> {
        todo!()
    }
    fn find(&self, x: &i32) -> Option<i32> {
        let mut i = 0;
        let mut c = 0;
        let ix = *x as usize;
        let mut u = self.r.clone();
        for _ in 0..BinaryTrie::W {
            let mut next = None;
            if let Some(ref u) = u {
                c = (ix >> (BinaryTrie::W - i - 1)) & 1;
                match *u.child[c].borrow() {
                    Some(ref c) => next = Some(c.clone()),
                    None => break,
                }
            }
            u = next;
            i += 1;
        }
        if i == BinaryTrie::W {
            return u.as_ref().map(|u| *u.x.borrow());
        }
        u = if c == 0 {
            u.as_ref().and_then(|u| u.jump.borrow().clone())
        } else {
            u.as_ref().and_then(|u| {
                let j = u.jump.borrow().clone();
                match j {
                    Some(ref j) => j.next.borrow().clone(),
                    None => None,
                }
            })
        };
        match u {
            Some(ref u) if u.next.borrow().is_none() => None,
            Some(ref u) if u.prev.borrow().is_none() => None,
            _ => u.as_ref().map(|u| *u.x.borrow()),
        }
    }
}
