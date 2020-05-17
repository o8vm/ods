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
pub struct Treap<T> {
    n: usize,
    r: Option<Rc<TreapNode<T>>>,
}

impl<T> TreapNode<T> {}

impl<T> Treap<T> {
    fn rotate_left(&mut self, u: &Rc<TreapNode<T>>) {
        let w = u.right.borrow_mut().take().unwrap(); // 絶対に None ではない
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
        let p = w.parent.borrow().as_ref().and_then(|p| p.upgrade());
        p.map(|p| {
            let left = p.left.borrow().clone();
            if let Some(ref left) = left {
                if Rc::ptr_eq(left, u) {
                    p.left.borrow_mut().replace(w.clone());
                } else {
                    p.right.borrow_mut().replace(w.clone());
                }
            }
        });
        *u.right.borrow_mut() = w.left.borrow_mut().take();
        if let Some(ref right) = *u.right.borrow() {
            right.parent.borrow_mut().replace(Rc::downgrade(u));
        }
        u.parent.borrow_mut().replace(Rc::downgrade(&w));
        w.left.borrow_mut().replace(u.clone());
        if Rc::ptr_eq(u, self.r.as_ref().unwrap()) {
            self.r.replace(w);
            // self.r.as_mut().map(|r| *r.parent.borrow_mut() = None);//<=いらないとおもう。
        }
    }
    fn rotate_right(&mut self, u: &Rc<TreapNode<T>>) {
        let w = u.left.borrow_mut().take().unwrap(); // 絶対に None ではない
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
        let p = w.parent.borrow().as_ref().and_then(|p| p.upgrade());
        p.map(|p| {
            let left = p.left.borrow().clone();
            if let Some(ref left) = left {
                if Rc::ptr_eq(left, u) {
                    p.left.borrow_mut().replace(w.clone());
                } else {
                    p.right.borrow_mut().replace(w.clone());
                }
            }
        });
        *u.left.borrow_mut() = w.right.borrow_mut().take();
        if let Some(ref left) = *u.left.borrow() {
            left.parent.borrow_mut().replace(Rc::downgrade(u));
        }
        u.parent.borrow_mut().replace(Rc::downgrade(&w));
        w.right.borrow_mut().replace(u.clone());
        if Rc::ptr_eq(u, self.r.as_ref().unwrap()) {
            self.r.replace(w);
            // self.r.as_mut().map(|r| *r.parent.borrow_mut() = None);//<=いらないとおもう。
        }
    }
    fn bubbleup(&mut self, u: &Rc<TreapNode<T>>) {
        loop {
            match u.parent.borrow().as_ref().and_then(|p| p.upgrade()) {
                Some(ref p) if *p.p.borrow() > *u.p.borrow() => {
                    p.right.borrow().as_ref().map(|r| {
                        if Rc::ptr_eq(r, u) {
                            self.rotate_left(p);
                        } else {
                            self.rotate_right(p);
                        }
                    });
                }
                _ => break,
            }
        }
        if u.parent.borrow().is_some() {
            self.r.replace(u.clone());
        }
    }
}
