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
pub struct ScapegoatTree<T> {
    n: usize,
    q: usize,
    r: Option<Rc<BSTNode<T>>>,
}

impl<T: Default> BSTNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<T> ScapegoatTree<T>
where
    T: Ord + Clone,
{
    fn size_u(u: &Tree<T>) -> usize {
        match u {
            Some(n) => 1 + Self::size_u(&n.left.borrow()) + Self::size_u(&n.right.borrow()),
            None => 0,
        }
    }
    fn build_balanced(mut a: &[Tree<T>], i: usize, ns: usize) -> Tree<T> {
        if ns == 0 {
            return None
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
                self.r.as_ref().map(|r| {
                    r.parent.borrow_mut().take();
                });
            },
            Some(p) => {
                let right = p.right.borrow().clone();
                match right {
                    Some(ref right) if Rc::ptr_eq(right, u.as_ref().unwrap())=> {
                        *p.right.borrow_mut() = Self::build_balanced(&a, 0, ns);
                        if let Some(right) = &*p.right.borrow() {
                            right.parent.borrow_mut().replace(Rc::downgrade(&p));
                        }
                    },
                    _ => {
                        *p.left.borrow_mut() = Self::build_balanced(&a, 0, ns);
                        if let Some(left) = &*p.left.borrow() {
                            left.parent.borrow_mut().replace(Rc::downgrade(&p));
                        }
                    },
                }
            },
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
    fn add_with_depth(&mut self, u: Rc<BSTNode<T>>) -> i32 {
        let mut w = self.r.clone();
        if let None = w {
            self.r = Some(u.clone());
        }
        let mut d = 0;
        let mut next;
        loop {
            match w {
                Some(ref w) => {
                    if &*u.x.borrow() < &*w.x.borrow() {
                        match &*w.left.borrow() {
                            None => {
                                *w.left.borrow_mut() = Some(u.clone());
                                *u.parent.borrow_mut() = Some(Rc::downgrade(w));
                                next = None;
                            },
                            Some(left) => next = Some(left.clone()),
                        }
                    } else if &*u.x.borrow() > &*w.x.borrow() {
                        match &*w.right.borrow() {
                            None => {
                                *w.right.borrow_mut() = Some(u.clone());
                                *u.parent.borrow_mut() = Some(Rc::downgrade(w));
                                next = None;
                            },
                            Some(right) => next = Some(right.clone()),
                        }
                    } else {
                        return -1
                    }
                    d += 1;
                },
                None => {
                    self.n += 1;
                    self.q += 1;
                    break d
                }
            }
            w = next;
        }
    }
}

impl<T> SSet<T> for ScapegoatTree<T>
where
    T: Ord + Clone + Default,
{
    fn size(&self) -> usize { 
        self.n
     }
    fn add(&mut self, x: T) -> bool { 
        let u = BSTNode::new(x);
        todo!() 
    }
    fn remove(&mut self, _: &T) -> std::option::Option<T> { todo!() }
    fn find(&self, _: &T) -> std::option::Option<T> { todo!() }
}
