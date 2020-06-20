#![allow(clippy::many_single_char_names)]
use crate::{xfasttrie::XFastTrie, USizeV};
use chapter01::interface::SSet;
use chapter07::treap::Treap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct YPair<T: USizeV + Default> {
    ix: usize,
    t: Rc<RefCell<Treap<T>>>,
}

impl<'a, T: USizeV + Default> USizeV for YPair<T> {
    fn usize_value(&self) -> usize {
        self.ix
    }
}

impl<'a, T> Default for YPair<T> 
where
    T: USizeV + Default + PartialOrd + Clone,
{
    fn default() -> Self {
        Self {
            ..Default::default()
        }
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
        //xft.add(YPair::with_x((1<<Self::W)-1));
        Self {
            n: 0,
            xft,
        }

    }
}

impl<T> SSet<T> for YFastTrie<T> 
where
    T: USizeV + Default + PartialOrd + Clone,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let ix = x.usize_value();
        let mut t = self.xft.find(&YPair::with_x(ix)).map(|y| y.t.clone());
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
            },
            None => false
        }
    }
    fn remove(&mut self, _: &T) -> std::option::Option<T> {
        todo!()
    }
    fn find(&self, x: &T) -> Option<T> {
        self.xft.find(&YPair::with_x(x.usize_value())).and_then(|y| y.t.borrow().find(&x))
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
        /*
        for _ in 0..5 {
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
        }
        */
    }
}