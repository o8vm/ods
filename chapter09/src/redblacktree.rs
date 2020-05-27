use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Color {
    Red,    // 0
    Black,  // 1
    WBlack, // 2
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}

type Tree<T> = Option<Rc<RBTNode<T>>>;

#[derive(Clone, Debug, Default)]
pub struct RBTNode<T> {
    color: RefCell<Color>,
    x: RefCell<T>,
    left: RefCell<Option<Rc<RBTNode<T>>>>,
    right: RefCell<Option<Rc<RBTNode<T>>>>,
    parent: RefCell<Option<Weak<RBTNode<T>>>>,
}

#[derive(Clone, Debug, Default)]
pub struct RedBlackTree<T> {
    n: usize,
    r: Option<Rc<RBTNode<T>>>,
}

impl<T: Default> RBTNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            x: RefCell::new(x),
            ..Default::default()
        }
    }
}

impl<T: Ord + Clone> RedBlackTree<T> {
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
    fn rotate_left(&mut self, u: &Rc<RBTNode<T>>) {
        let w = u.right.borrow_mut().take().unwrap();
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
        let p = w.parent.borrow().as_ref().and_then(|p| p.upgrade());
        p.map(|p| {
            let left = p.left.borrow().clone();
            match left {
                Some(ref left) if Rc::ptr_eq(left, u) => {
                    p.left.borrow_mut().replace(w.clone());
                }
                _ => {
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
        }
    }
    fn rotate_right(&mut self, u: &Rc<RBTNode<T>>) {
        let w = u.left.borrow_mut().take().unwrap();
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
        let p = w.parent.borrow().as_ref().and_then(|p| p.upgrade());
        p.map(|p| {
            let left = p.left.borrow().clone();
            match left {
                Some(ref left) if Rc::ptr_eq(left, u) => {
                    p.left.borrow_mut().replace(w.clone());
                }
                _ => {
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
        }
    }
    fn swap_colors(u: &Rc<RBTNode<T>>, w: &Rc<RBTNode<T>>) {
        u.color.swap(&w.color);
    }
    fn push_black(u: &Rc<RBTNode<T>>) {
        *u.color.borrow_mut() = Color::Red;
        u.left
            .borrow()
            .as_ref()
            .map(|left| *left.color.borrow_mut() = Color::Black);
        u.right
            .borrow()
            .as_ref()
            .map(|right| *right.color.borrow_mut() = Color::Black);
    }
    fn pull_black(u: &Rc<RBTNode<T>>) {
        *u.color.borrow_mut() = Color::Black;
        u.left
            .borrow()
            .as_ref()
            .map(|left| *left.color.borrow_mut() = Color::Red);
        u.right
            .borrow()
            .as_ref()
            .map(|right| *right.color.borrow_mut() = Color::Red);
    }
    fn flip_left(&mut self, u: &Rc<RBTNode<T>>) {
        Self::swap_colors(u, u.right.borrow().as_ref().unwrap());
        self.rotate_left(u);
    }
    fn flip_right(&mut self, u: &Rc<RBTNode<T>>) {
        Self::swap_colors(u, u.left.borrow().as_ref().unwrap());
        self.rotate_right(u);
    }
    fn add_child(&mut self, p: &Tree<T>, u: Rc<RBTNode<T>>) -> bool {
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
    fn add_u(&mut self, u: Rc<RBTNode<T>>) -> bool {
        let p = self.find_last(&*u.x.borrow());
        self.add_child(&p, u)
    }
    fn add_fixup(&mut self, mut u: Rc<RBTNode<T>>) {
        while *u.color.borrow() == Color::Red {
            if Rc::ptr_eq(&u, self.r.as_ref().unwrap()) {
                *u.color.borrow_mut() = Color::Black;
                break;
            }
            let mut w = u
                .parent
                .borrow()
                .as_ref()
                .and_then(|p| p.upgrade())
                .unwrap();
            if w.left.borrow().as_ref().map(|left| *left.color.borrow()) == Some(Color::Black) {
                self.flip_left(&w);
                u = w;
                w = u
                    .parent
                    .borrow()
                    .as_ref()
                    .and_then(|p| p.upgrade())
                    .unwrap();
            }
            if *w.color.borrow() == Color::Black {
                break;
            }
            let g = w
                .parent
                .borrow()
                .as_ref()
                .and_then(|p| p.upgrade())
                .unwrap();
            if g.right.borrow().as_ref().map(|right| *right.color.borrow()) == Some(Color::Black) {
                self.flip_right(&g);
                break;
            } else {
                Self::push_black(&g);
                u = g;
            }
        }
    }
    fn splice(&mut self, u: Rc<RBTNode<T>>) -> Option<T> {
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
                p.as_ref().map(|p| {
                    let left = p.left.borrow().clone();
                    match left {
                        Some(ref left) if Rc::ptr_eq(left, &u) => {
                            *p.left.borrow_mut() = s.clone();
                        }
                        _ => {
                            *p.right.borrow_mut() = s.clone();
                        }
                    }
                });
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
    fn remove_fixup(&mut self, mut u: Rc<RBTNode<T>>) {
        while *u.color.borrow() == Color::WBlack {
            if Rc::ptr_eq(&u, self.r.as_ref().unwrap()) {
                *u.color.borrow_mut() = Color::Black;
            } else {
                let mut w = u.parent.borrow().as_ref().and_then(|p| p.upgrade()).unwrap();
                let left = w.left.borrow().clone();
                match left {
                    Some(ref left) if *left.color.borrow() == Color::Red => todo!(),
                    Some(ref left) if Rc::ptr_eq(&u, left) => todo!(),
                    _ => todo!(),
                }
            }
        }
        todo!()
    }
    fn remove_fix_case1(&mut self, u: Rc<RBTNode<T>>) -> Rc<RBTNode<T>> {
        todo!()
    }
    fn remove_fix_case2(&mut self, u: Rc<RBTNode<T>>) -> Rc<RBTNode<T>> {
        todo!()
    }
    fn remove_fix_case3(&mut self, u: Rc<RBTNode<T>>) -> Rc<RBTNode<T>> {
        todo!()
    }
}

impl<T> SSet<T> for RedBlackTree<T>
where
    T: Ord + Clone + Default,
{
    fn size(&self) -> usize {
        self.n
    }
    fn add(&mut self, x: T) -> bool {
        let u = Rc::new(RBTNode::new(x));
        let added = self.add_u(u.clone());
        if added {
            self.add_fixup(u);
        }
        added
    }
    fn remove(&mut self, x: &T) -> Option<T> {
        match self.find_last(x) {
            Some(u) if &*u.x.borrow() == x => {
                let mut w = u.right.borrow().clone();
                let wi;
                let ui;
                if w.is_none() {
                    wi = u;
                    ui = wi.left.borrow().clone()?;
                } else {
                    while w.as_ref()?.left.borrow().is_some() {
                        w = w.as_ref().and_then(|w| w.left.borrow().clone());
                    }
                    wi = w?;
                    u.x.swap(&wi.x);
                    ui = wi.right.borrow().clone()?;
                }
                let test = *ui.color.borrow() as isize + *wi.color.borrow() as isize;
                *ui.color.borrow_mut() = match test {
                    0 => Color::Red,
                    1 => Color::Black,
                    _ => Color::WBlack,
                };
                ui.parent.swap(&wi.parent);
                let res = self.splice(wi);
                self.remove_fixup(ui);
                res
            }
            _ => None,
        }
    }
    fn find(&self, _: &T) -> std::option::Option<T> {
        todo!()
    }
}
