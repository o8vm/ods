#![allow(clippy::many_single_char_names)]
use crate::{xfasttrie::XFastTrie, USizeV};
use chapter01::interface::SSet;
use chapter07::treap::Treap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct YPair<T: USizeV + Default + Clone + PartialOrd> {
    ix: usize,
    t: Rc<RefCell<Treap<T>>>,
}

impl<T: USizeV + Default + Clone + PartialOrd> USizeV for YPair<T> {
    fn usize_value(&self) -> usize {
        self.ix
    }
}

impl<T> PartialOrd for YPair<T>
where
    T: USizeV + Default + PartialOrd + Clone,
{
    fn partial_cmp(&self, other: &YPair<T>) -> Option<std::cmp::Ordering> {
        Some(self.ix.cmp(&other.ix))
    }
}

impl<T> PartialEq for YPair<T>
where
    T: USizeV + Default + PartialOrd + Clone,
{
    fn eq(&self, other: &YPair<T>) -> bool {
        self.ix == other.ix
    }
}

impl<T> YPair<T>
where
    T: USizeV + Default + PartialOrd + Clone,
{
    fn with_x(ix: usize) -> Self {
        Self {
            ix,
            t: Rc::new(RefCell::new(Treap::new())),
        }
    }
    fn with_xt(ix: usize, t: Treap<T>) -> Self {
        Self {
            ix,
            t: Rc::new(RefCell::new(t)),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct YFastTrie<T>
where
    T: USizeV + Default + PartialOrd + Clone,
{
    xft: XFastTrie<YPair<T>>,
    n: usize,
}

impl<T> YFastTrie<T>
where
    T: USizeV + Default + PartialOrd + Clone,
{
    const W: usize = 32;
    pub fn new() -> Self {
        let mut xft = XFastTrie::new();
        xft.add(YPair::with_x((1 << Self::W) - 1));
        Self { n: 0, xft }
    }
}

impl<T> SSet<T> for YFastTrie<T>
where
    T: USizeV + Default + PartialOrd + Clone + std::fmt::Debug,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let ix = x.usize_value();
        let mut t = self.xft.find(&YPair::with_x(ix)).map(|y| y.t);
        match t {
            Some(ref mut t) => {
                if t.borrow_mut().add(x.clone()) {
                    self.n += 1;
                    if rand::random::<usize>() % Self::W == 0 {
                        let t1 = t.borrow_mut().split(x);
                        self.xft.add(YPair::with_xt(ix, t1));
                    }
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        let ix = x.usize_value();
        let u = self.xft.find_node(ix);
        let ret = match &u {
            Some(u) => u.x.borrow().t.borrow_mut().remove(x),
            None => None,
        };
        if ret.is_some() {
            self.n -= 1;
        }
        if let Some(u) = u {
            if u.x.borrow().ix == ix && ix != (1 << Self::W) - 1 {
                if let Some(n) = u.next.borrow().as_ref() {
                    n.x.borrow()
                        .t
                        .borrow_mut()
                        .absorb(u.x.borrow().t.replace(Treap::new()));
                }
                self.xft.remove_node(u);
            }
        }
        ret
    }
    fn find(&self, x: &T) -> Option<T> {
        self.xft
            .find(&YPair::with_x(x.usize_value()))
            .and_then(|y| y.t.borrow().find(&x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::SSet;
    use chapter09::redblacktree::RedBlackTree;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_yfasttrie() {
        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut yfasttrie = YFastTrie::<i32>::new();

        for _ in 0..2 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                yfasttrie.add(x);
                assert_eq!(redblacktree.size(), yfasttrie.size());
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = yfasttrie.find(&x);
                assert_eq!(y1, y2);
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let b1 = redblacktree.remove(&x);
                let b2 = yfasttrie.remove(&x);
                assert_eq!(b1, b2);
            }
            assert_eq!(redblacktree.size(), yfasttrie.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = yfasttrie.find(&x);
                assert_eq!(y1, y2);
            }
        }
        // test large linked list for stack overflow.
        let mut bst = YFastTrie::<i32>::new();
        let num = 100000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
