#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Tree<T> = Option<Rc<TreapNode<T>>>;

#[derive(Clone, Debug, Default)]
pub struct TreapNode<T> {
    p: RefCell<usize>,
    x: RefCell<T>,
    left: RefCell<Option<Rc<TreapNode<T>>>>,
    right: RefCell<Option<Rc<TreapNode<T>>>>,
    parent: RefCell<Option<Weak<TreapNode<T>>>>,
}

#[derive(Clone, Debug, Default)]
pub struct Treap<T: PartialOrd + Clone> {
    n: usize,
    r: Option<Rc<TreapNode<T>>>,
}

impl<T: PartialOrd + Clone> Drop for Treap<T> {
    fn drop(&mut self) {
        while let Some(r) = self.r.clone() {
            self.splice(r);
        }
    }
}

impl<T: Default> TreapNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            x: RefCell::new(x),
            ..Default::default()
        }
    }
}

impl<T> Treap<T>
where
    T: PartialOrd + Clone,
{
    pub fn new() -> Self {
        Self { n: 0, r: None }
    }
    fn rotate_left(&mut self, u: &Rc<TreapNode<T>>) {
        let w = u.right.borrow_mut().take().unwrap();
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
        let p = w.parent.borrow().as_ref().and_then(|p| p.upgrade());
        if let Some(p) = p {
            let left = p.left.borrow().clone();
            match left {
                Some(ref left) if Rc::ptr_eq(left, u) => {
                    p.left.borrow_mut().replace(w.clone());
                }
                _ => {
                    p.right.borrow_mut().replace(w.clone());
                }
            }
        }
        *u.right.borrow_mut() = w.left.borrow_mut().take();
        if let Some(ref right) = *u.right.borrow() {
            right.parent.borrow_mut().replace(Rc::downgrade(u));
        }
        u.parent.borrow_mut().replace(Rc::downgrade(&w));
        w.left.borrow_mut().replace(u.clone());
        if Rc::ptr_eq(u, self.r.as_ref().unwrap()) {
            self.r.replace(w);
        }
    }
    fn rotate_right(&mut self, u: &Rc<TreapNode<T>>) {
        let w = u.left.borrow_mut().take().unwrap();
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
        let p = w.parent.borrow().as_ref().and_then(|p| p.upgrade());
        if let Some(p) = p {
            let left = p.left.borrow().clone();
            match left {
                Some(ref left) if Rc::ptr_eq(left, u) => {
                    p.left.borrow_mut().replace(w.clone());
                }
                _ => {
                    p.right.borrow_mut().replace(w.clone());
                }
            }
        }
        *u.left.borrow_mut() = w.right.borrow_mut().take();
        if let Some(ref left) = *u.left.borrow() {
            left.parent.borrow_mut().replace(Rc::downgrade(u));
        }
        u.parent.borrow_mut().replace(Rc::downgrade(&w));
        w.right.borrow_mut().replace(u.clone());
        if Rc::ptr_eq(u, self.r.as_ref().unwrap()) {
            self.r.replace(w);
        }
    }
    fn bubbleup(&mut self, u: &Rc<TreapNode<T>>) {
        loop {
            let parent = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
            match parent {
                Some(ref p) if *p.p.borrow() > *u.p.borrow() => {
                    let right = p.right.borrow().clone();
                    match right {
                        Some(ref r) if Rc::ptr_eq(r, u) => self.rotate_left(p),
                        _ => self.rotate_right(p),
                    }
                }
                _ => break,
            }
        }
        if u.parent.borrow().is_none() {
            self.r.replace(u.clone());
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
    fn add_child(&mut self, p: &Tree<T>, u: Rc<TreapNode<T>>) -> bool {
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
    fn add_u(&mut self, u: Rc<TreapNode<T>>) -> bool {
        let p = self.find_last(&*u.x.borrow());
        self.add_child(&p, u)
    }
    fn splice(&mut self, u: Rc<TreapNode<T>>) -> Option<T> {
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
    fn trickle_down(&mut self, u: &Rc<TreapNode<T>>) {
        while u.left.borrow().is_some() || u.right.borrow().is_some() {
            let left = u.left.borrow().clone();
            let right = u.right.borrow().clone();
            match (left, right) {
                (None, _) => self.rotate_left(u),
                (_, None) => self.rotate_right(u),
                (Some(l), Some(r)) if *l.p.borrow() < *r.p.borrow() => self.rotate_right(u),
                _ => self.rotate_left(u),
            }
            if Rc::ptr_eq(u, self.r.as_ref().unwrap()) {
                let p = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
                self.r = p;
            }
        }
    }
}

impl<T> Treap<T>
where
    T: PartialOrd + Clone + Default,
{
    pub fn split(&mut self, x: T) -> Treap<T> {
        let mut u = self.find_last(&x);
        let s = Rc::new(TreapNode::<T>::new(Default::default()));
        match u {
            Some(ref u) if u.right.borrow().is_none() => {
                u.right.borrow_mut().replace(s.clone());
            }
            _ => {
                let ur = u.as_ref().and_then(|u| u.right.borrow().clone());
                u = ur;
                while let Some(v) = u.as_ref().and_then(|u| u.left.borrow().clone()) {
                    u = Some(v);
                }
                if let Some(u) = u.as_ref() {
                    u.left.borrow_mut().replace(s.clone());
                }
            }
        }
        *s.parent.borrow_mut() = u.as_ref().map(|u| Rc::downgrade(&u));
        *s.p.borrow_mut() = usize::MIN;
        self.bubbleup(&s);
        self.r = s.right.borrow_mut().take();
        if let Some(ref r) = self.r {
            *r.parent.borrow_mut() = None;
        }
        let mut ret = Treap::<T>::new();
        ret.r = s.left.borrow_mut().take();
        if let Some(ref r) = ret.r {
            *r.parent.borrow_mut() = None;
        }
        ret.n = self.n;
        ret
    }
    pub fn absorb(&mut self, mut t: Treap<T>) {
        let s = Rc::new(TreapNode::<T>::new(Default::default()));
        *s.right.borrow_mut() = self.r.clone();
        if let Some(r) = self.r.take() {
            r.parent.borrow_mut().replace(Rc::downgrade(&s));
        }
        *s.left.borrow_mut() = t.r.clone();
        if let Some(r2) = t.r.take() {
            r2.parent.borrow_mut().replace(Rc::downgrade(&s));
        }
        self.r.replace(s.clone());
        self.trickle_down(&s);
        self.splice(s);
    }
}

impl<T> SSet<T> for Treap<T>
where
    T: PartialOrd + Clone + Default,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let u = Rc::new(TreapNode::new(x));
        *u.p.borrow_mut() = rand::random();
        if self.add_u(u.clone()) {
            self.bubbleup(&u);
            true
        } else {
            false
        }
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        match self.find_last(x) {
            Some(u) if &*u.x.borrow() == x => {
                self.trickle_down(&u);
                self.splice(u)
            }
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
    use chapter09::redblacktree::RedBlackTree;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_treap() {
        let mut treap = Treap::<u32>::new();
        treap.add(3);
        treap.add(1);
        treap.add(5);
        treap.add(0);
        treap.add(2);
        treap.add(4);
        treap.add(9);
        treap.add(7);
        treap.add(6);
        treap.add(8);
        assert_eq!(false, treap.add(8));
        assert_eq!(Some(3), treap.find(&3));
        assert_eq!(None, treap.find(&10));
        assert_eq!(Some(9), treap.remove(&9));
        assert_eq!(Some(8), treap.remove(&8));
        assert_eq!(None, treap.remove(&8));
        //println!("{:?}", treap);
        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut treap = Treap::<i32>::new();

        for _ in 0..5 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                treap.add(x);
                assert_eq!(redblacktree.size(), treap.size());
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = treap.find(&x);
                assert_eq!(y1, y2);
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let b1 = redblacktree.remove(&x);
                let b2 = treap.remove(&x);
                assert_eq!(b1, b2);
            }
            assert_eq!(redblacktree.size(), treap.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = treap.find(&x);
                assert_eq!(y1, y2);
            }
        }

        // test large linked list for stack overflow.
        let mut bst = Treap::<i32>::new();
        let num = 100000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
