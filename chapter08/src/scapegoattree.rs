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
pub struct ScapegoatTree<T: PartialOrd + Clone> {
    n: usize,
    q: usize,
    r: Option<Rc<BSTNode<T>>>,
}

impl<T: PartialOrd + Clone> Drop for ScapegoatTree<T> {
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

impl<T> ScapegoatTree<T>
where
    T: PartialOrd + Clone,
{
    pub fn new() -> Self {
        Self {
            n: 0,
            q: 0,
            r: None,
        }
    }
    fn size_u(u: &Tree<T>) -> usize {
        match u {
            Some(n) => 1 + Self::size_u(&n.left.borrow()) + Self::size_u(&n.right.borrow()),
            None => 0,
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
    fn build_balanced(a: &[Tree<T>], i: usize, ns: usize) -> Tree<T> {
        if ns == 0 {
            return None;
        }
        let m = ns / 2;
        let elem = a[i + m].clone();
        if let Some(ref u) = elem {
            *u.left.borrow_mut() = Self::build_balanced(a, i, m);
            if let Some(left) = &*u.left.borrow() {
                left.parent.borrow_mut().replace(Rc::downgrade(u));
            }
            *u.right.borrow_mut() = Self::build_balanced(a, i + m + 1, ns - m - 1);
            if let Some(right) = &*u.right.borrow() {
                right.parent.borrow_mut().replace(Rc::downgrade(u));
            }
        }
        elem
    }
    fn rebuild(&mut self, u: &Tree<T>) {
        let ns = Self::size_u(u);
        let p = match u {
            Some(u) => u.parent.borrow().as_ref().and_then(|p| p.upgrade()),
            None => None,
        };
        let mut a: Vec<Tree<T>> = vec![None; ns];
        Self::pack_into_array(u, &mut a, 0);
        match p {
            None => {
                self.r = Self::build_balanced(&a, 0, ns);
                if let Some(r) = self.r.as_ref() {
                    r.parent.borrow_mut().take();
                }
            }
            Some(p) => {
                let right = p.right.borrow().clone();
                match right {
                    Some(ref right) if Rc::ptr_eq(right, u.as_ref().unwrap()) => {
                        *p.right.borrow_mut() = Self::build_balanced(&a, 0, ns);
                        if let Some(right) = &*p.right.borrow() {
                            right.parent.borrow_mut().replace(Rc::downgrade(&p));
                        }
                    }
                    _ => {
                        *p.left.borrow_mut() = Self::build_balanced(&a, 0, ns);
                        if let Some(left) = &*p.left.borrow() {
                            left.parent.borrow_mut().replace(Rc::downgrade(&p));
                        }
                    }
                }
            }
        }
    }
    fn pack_into_array(u: &Tree<T>, a: &mut [Tree<T>], mut i: usize) -> usize {
        match u {
            None => i,
            Some(u) => {
                i = Self::pack_into_array(&u.left.borrow(), a, i);
                if let Some(elem) = a.get_mut(i) {
                    elem.replace(u.clone());
                }
                i += 1;
                Self::pack_into_array(&u.right.borrow(), a, i)
            }
        }
    }
    fn add_with_depth(&mut self, u: Rc<BSTNode<T>>) -> i64 {
        let mut w = self.r.clone();
        if w.is_none() {
            self.r = Some(u.clone());
        }
        let mut d = 0;
        let mut next;
        loop {
            match w {
                Some(ref w) => {
                    if *u.x.borrow() < *w.x.borrow() {
                        let left = w.left.borrow().clone();
                        match left {
                            None => {
                                *w.left.borrow_mut() = Some(u.clone());
                                *u.parent.borrow_mut() = Some(Rc::downgrade(w));
                                next = None;
                            }
                            Some(left) => next = Some(left.clone()),
                        }
                    } else if *u.x.borrow() > *w.x.borrow() {
                        let right = w.right.borrow().clone();
                        match right {
                            None => {
                                *w.right.borrow_mut() = Some(u.clone());
                                *u.parent.borrow_mut() = Some(Rc::downgrade(w));
                                next = None;
                            }
                            Some(right) => next = Some(right.clone()),
                        }
                    } else {
                        return -1;
                    }
                    d += 1;
                }
                None => {
                    self.n += 1;
                    self.q += 1;
                    break d;
                }
            }
            w = next;
        }
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
    fn bst_remove(&mut self, x: &T) -> Option<T> {
        match self.find_last(x) {
            Some(u) if &*u.x.borrow() == x => self.remove_u(u),
            _ => None,
        }
    }
}

impl<T> SSet<T> for ScapegoatTree<T>
where
    T: PartialOrd + Clone + Default,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let u = Rc::new(BSTNode::new(x));
        let d = self.add_with_depth(u.clone());
        if d > crate::log32(self.q) {
            let mut w = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
            let mut wp = w
                .as_ref()
                .and_then(|w| w.parent.borrow().as_ref().and_then(|wp| wp.upgrade()));
            let mut a = Self::size_u(&w);
            let mut b = Self::size_u(&wp);
            while 3 * a <= 2 * b {
                w = wp;
                wp = w
                    .as_ref()
                    .and_then(|w| w.parent.borrow().as_ref().and_then(|wp| wp.upgrade()));
                a = Self::size_u(&w);
                b = Self::size_u(&wp);
            }
            self.rebuild(&wp);
            true
        } else {
            d >= 0
        }
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        match self.bst_remove(x) {
            Some(x) => {
                if 2 * self.n < self.q {
                    if self.r.is_some() {
                        self.rebuild(&self.r.clone());
                    }
                    self.q = self.n;
                }
                Some(x)
            }
            None => None,
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
    fn test_scapegoattree() {
        let mut scapegoattree = ScapegoatTree::<u32>::new();
        scapegoattree.add(9);
        scapegoattree.add(8);
        scapegoattree.add(10);
        scapegoattree.add(7);
        scapegoattree.add(11);
        scapegoattree.add(3);
        scapegoattree.add(1);
        scapegoattree.add(6);
        scapegoattree.add(0);
        scapegoattree.add(4);
        scapegoattree.add(5);
        let u = scapegoattree.find_last(&4).unwrap();
        let p = u
            .parent
            .borrow()
            .as_ref()
            .and_then(|p| p.upgrade())
            .unwrap();
        assert_eq!(8, *p.x.borrow());
        assert_eq!(1, *u.left.borrow().as_ref().unwrap().x.borrow());
        assert_eq!(6, *u.right.borrow().as_ref().unwrap().x.borrow());
        let u = scapegoattree.find_last(&1).unwrap();
        let p = u
            .parent
            .borrow()
            .as_ref()
            .and_then(|p| p.upgrade())
            .unwrap();
        assert_eq!(4, *p.x.borrow());
        assert_eq!(0, *u.left.borrow().as_ref().unwrap().x.borrow());
        assert_eq!(3, *u.right.borrow().as_ref().unwrap().x.borrow());
        let u = scapegoattree.find_last(&6).unwrap();
        let p = u
            .parent
            .borrow()
            .as_ref()
            .and_then(|p| p.upgrade())
            .unwrap();
        assert_eq!(4, *p.x.borrow());
        assert_eq!(5, *u.left.borrow().as_ref().unwrap().x.borrow());
        assert_eq!(7, *u.right.borrow().as_ref().unwrap().x.borrow());
        let mut scapegoattree = ScapegoatTree::<u32>::new();
        scapegoattree.add(9);
        scapegoattree.add(8);
        scapegoattree.add(10);
        scapegoattree.add(7);
        scapegoattree.add(11);
        scapegoattree.add(3);
        scapegoattree.add(1);
        scapegoattree.add(6);
        scapegoattree.add(0);
        scapegoattree.add(4);
        scapegoattree.add(5);
        assert_eq!(Some(5), scapegoattree.remove(&5));
        assert_eq!(None, scapegoattree.remove(&5));
        assert_eq!(Some(8), scapegoattree.remove(&8));
        assert_eq!(None, scapegoattree.remove(&8));
        assert_eq!(Some(9), scapegoattree.remove(&9));
        assert_eq!(Some(10), scapegoattree.remove(&10));
        assert_eq!(Some(4), scapegoattree.remove(&4));
        assert_eq!(Some(1), scapegoattree.remove(&1));
        println!("{:?}", scapegoattree);

        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut scapegoattree = ScapegoatTree::<i32>::new();

        for _ in 0..5 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                scapegoattree.add(x);
                assert_eq!(redblacktree.size(), scapegoattree.size());
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = scapegoattree.find(&x);
                assert_eq!(y1, y2);
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let b1 = redblacktree.remove(&x);
                let b2 = scapegoattree.remove(&x);
                assert_eq!(b1, b2);
            }
            assert_eq!(redblacktree.size(), scapegoattree.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = scapegoattree.find(&x);
                assert_eq!(y1, y2);
            }
        }

        // test large linked list for stack overflow.
        let mut bst = ScapegoatTree::<i32>::new();
        let num = 100000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
