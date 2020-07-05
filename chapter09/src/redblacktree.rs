#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
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
pub struct RedBlackTree<T: PartialOrd + Clone> {
    n: usize,
    r: Option<Rc<RBTNode<T>>>,
}

impl<T: PartialOrd + Clone> Drop for RedBlackTree<T> {
    fn drop(&mut self) {
        while let Some(r) = self.r.clone() {
            self.splice(r);
        }
    }
}

impl<T: Default> RBTNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            x: RefCell::new(x),
            ..Default::default()
        }
    }
}

impl<T: PartialOrd + Clone> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { n: 0, r: None }
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
    fn rotate_left(&mut self, u: &Rc<RBTNode<T>>) {
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
    fn rotate_right(&mut self, u: &Rc<RBTNode<T>>) {
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
    fn swap_colors(u: &Rc<RBTNode<T>>, w: &Rc<RBTNode<T>>) {
        u.color.swap(&w.color);
    }
    fn push_black(u: &Rc<RBTNode<T>>) {
        let uc = *u.color.borrow() as isize - 1;
        *u.color.borrow_mut() = match uc {
            0 => Color::Red,
            1 => Color::Black,
            _ => Color::WBlack,
        };
        if let Some(left) = u.left.borrow().as_ref() {
            let lc = *left.color.borrow() as isize + 1;
            *left.color.borrow_mut() = match lc {
                0 => Color::Red,
                1 => Color::Black,
                _ => Color::WBlack,
            };
        }
        if let Some(right) = u.right.borrow().as_ref() {
            let rc = *right.color.borrow() as isize + 1;
            *right.color.borrow_mut() = match rc {
                0 => Color::Red,
                1 => Color::Black,
                _ => Color::WBlack,
            }
        }
    }
    fn pull_black(u: &Rc<RBTNode<T>>) {
        let uc = *u.color.borrow() as isize + 1;
        *u.color.borrow_mut() = match uc {
            0 => Color::Red,
            1 => Color::Black,
            _ => Color::WBlack,
        };
        if let Some(left) = u.left.borrow().as_ref() {
            let lc = *left.color.borrow() as isize - 1;
            *left.color.borrow_mut() = match lc {
                0 => Color::Red,
                1 => Color::Black,
                _ => Color::WBlack,
            };
        }
        if let Some(right) = u.right.borrow().as_ref() {
            let rc = *right.color.borrow() as isize - 1;
            *right.color.borrow_mut() = match rc {
                0 => Color::Red,
                1 => Color::Black,
                _ => Color::WBlack,
            }
        }
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
            let mut w = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
            let left = w.as_ref().and_then(|w| w.left.borrow().clone());
            if let Some(Color::Black) | None = left.as_ref().map(|left| *left.color.borrow()) {
                self.flip_left(w.as_ref().unwrap());
                u = w.unwrap();
                w = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
            }
            if let Some(Color::Black) | None = w.as_ref().map(|w| *w.color.borrow()) {
                break;
            }
            let g = match w {
                Some(ref w) => match &*w.parent.borrow() {
                    Some(ref p) => p.upgrade(),
                    None => None,
                },
                None => None,
            };
            let gr = g.as_ref().and_then(|g| g.right.borrow().clone());
            if let Some(Color::Black) | None = gr.as_ref().map(|right| *right.color.borrow()) {
                self.flip_right(g.as_ref().unwrap());
                break;
            } else {
                Self::push_black(g.as_ref().unwrap());
                u = g.unwrap();
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
    fn remove_fixup(&mut self, mut color: isize, mut u: Tree<T>, mut p: Tree<T>) {
        while color > 1 {
            let n = u.clone();
            match n {
                Some(ref u) if Rc::ptr_eq(u, self.r.as_ref().unwrap()) => {
                    *u.color.borrow_mut() = Color::Black;
                    color = Color::Black as isize;
                }
                None if self.r.is_none() => {
                    color = Color::Black as isize;
                }
                _ => {
                    let left = p.as_ref().and_then(|p| p.left.borrow().clone());
                    match left {
                        Some(ref left) if *left.color.borrow() == Color::Red => {
                            let result = self.remove_fix_case1(u, p, color);
                            color = result.0;
                            u = result.1;
                            p = result.2;
                        }
                        Some(ref left)
                            if u.as_ref()
                                .map(|u| Rc::ptr_eq(u, left))
                                .filter(|b| b == &true)
                                .is_some() =>
                        {
                            let result = self.remove_fix_case2(p);
                            color = result.0;
                            u = result.1;
                            p = result.2;
                        }
                        None if u.is_none() => {
                            let result = self.remove_fix_case2(p);
                            color = result.0;
                            u = result.1;
                            p = result.2;
                        }
                        _ => {
                            let result = self.remove_fix_case3(p);
                            color = result.0;
                            u = result.1;
                            p = result.2;
                        }
                    }
                }
            }
        }
        if u.as_ref()
            .map(|u| !Rc::ptr_eq(u, self.r.as_ref().unwrap()))
            .filter(|b| b == &true)
            .is_some()
            || u.is_none()
        {
            let left = p.as_ref().and_then(|w| w.left.borrow().clone());
            let right = p.as_ref().and_then(|w| w.right.borrow().clone());
            match (left, right) {
                (Some(left), Some(right))
                    if *right.color.borrow() == Color::Red
                        && *left.color.borrow() == Color::Black =>
                {
                    self.flip_left(p.as_ref().unwrap())
                }
                (None, Some(right)) if *right.color.borrow() == Color::Red => {
                    self.flip_left(p.as_ref().unwrap())
                }
                _ => (),
            }
        }
    }
    fn remove_fix_case1(
        &mut self,
        u: Tree<T>,
        w: Tree<T>,
        color: isize,
    ) -> (isize, Tree<T>, Tree<T>) {
        self.flip_right(w.as_ref().unwrap());
        (color, u, w)
    }
    fn remove_fix_case2(&mut self, w: Tree<T>) -> (isize, Tree<T>, Tree<T>) {
        let v = w.as_ref().and_then(|w| w.right.borrow().clone());
        Self::pull_black(w.as_ref().unwrap()); // color -= 1;
        self.flip_left(w.as_ref().unwrap());
        let q = w.as_ref().and_then(|w| w.right.borrow().clone());
        if q.as_ref().map(|q| *q.color.borrow()) == Some(Color::Red) {
            self.rotate_left(w.as_ref().unwrap());
            self.flip_right(v.as_ref().unwrap());
            Self::push_black(q.as_ref().unwrap());
            let vr = v.as_ref().and_then(|v| v.right.borrow().clone());
            if vr.as_ref().map(|vr| *vr.color.borrow()) == Some(Color::Red) {
                self.flip_left(v.as_ref().unwrap());
            }
            let color = if let Some(q) = &q {
                *q.color.borrow() as isize
            } else {
                1
            };
            let p = q
                .as_ref()
                .and_then(|q| q.parent.borrow().as_ref().and_then(|p| p.upgrade()));
            (color, q, p)
        } else {
            let color = if let Some(v) = &v {
                *v.color.borrow() as isize
            } else {
                1
            };
            let p = v
                .as_ref()
                .and_then(|v| v.parent.borrow().as_ref().and_then(|p| p.upgrade()));
            (color, v, p)
        }
    }
    fn remove_fix_case3(&mut self, w: Tree<T>) -> (isize, Tree<T>, Tree<T>) {
        let v = w.as_ref().and_then(|w| w.left.borrow().clone());
        Self::pull_black(w.as_ref().unwrap());
        self.flip_right(w.as_ref().unwrap());
        let q = w.as_ref().and_then(|w| w.left.borrow().clone());
        if q.as_ref().map(|q| *q.color.borrow()) == Some(Color::Red) {
            self.rotate_right(w.as_ref().unwrap());
            self.flip_left(v.as_ref().unwrap());
            Self::push_black(q.as_ref().unwrap());
            let color = if let Some(q) = &q {
                *q.color.borrow() as isize
            } else {
                1
            };
            let p = q
                .as_ref()
                .and_then(|q| q.parent.borrow().as_ref().and_then(|p| p.upgrade()));
            (color, q, p)
        } else {
            let vl = v.as_ref().and_then(|v| v.left.borrow().clone());
            if vl.as_ref().map(|vl| *vl.color.borrow()) == Some(Color::Red) {
                Self::push_black(v.as_ref().unwrap());
                let color = if let Some(v) = &v {
                    *v.color.borrow() as isize
                } else {
                    1
                };
                let p = v
                    .as_ref()
                    .and_then(|v| v.parent.borrow().as_ref().and_then(|p| p.upgrade()));
                (color, v, p)
            } else {
                self.flip_left(v.as_ref().unwrap());
                let color = if let Some(w) = &w {
                    *w.color.borrow() as isize
                } else {
                    1
                };
                let p = w
                    .as_ref()
                    .and_then(|w| w.parent.borrow().as_ref().and_then(|p| p.upgrade()));
                (color, w, p)
            }
        }
    }

    pub fn is_a_valid_red_black_tree(&self) -> bool {
        let result = self.validade(&self.r, Color::Red, 0);
        let red_red = result.0;
        let black_hight_min = result.1;
        let black_height_max = result.2;
        let left_leaning = result.3;
        /*
        println!(
            "red_red={}, black_hight_min={}, black_height_max={}, left_leaning={}",
            red_red, black_hight_min, black_height_max, left_leaning
        );
        */
        red_red == 0 && black_hight_min == black_height_max && left_leaning
    }
    fn validade(
        &self,
        node: &Tree<T>,
        parent_color: Color,
        black_height: usize,
    ) -> (usize, usize, usize, bool) {
        // black_height: min black-height == max black-height
        // red_red: u.color + u.parent.color >= 1
        // left-leaning: if u.left is black, u.right is black.
        if let Some(u) = node {
            let red_red = if parent_color == Color::Red && *u.color.borrow() == Color::Red {
                1
            } else {
                0
            };
            let black_height = black_height
                + match *u.color.borrow() {
                    Color::Black => 1,
                    Color::Red => 0,
                    Color::WBlack => panic!(),
                };
            let left = &*u.left.borrow();
            let right = &*u.right.borrow();
            let left_leaning = match (left, right) {
                (Some(left), Some(right))
                    if *left.color.borrow() == Color::Black
                        && *right.color.borrow() == Color::Red =>
                {
                    false
                }
                (None, Some(right)) if *right.color.borrow() == Color::Red => false,
                _ => true,
            };
            let l = self.validade(&u.left.borrow(), *u.color.borrow(), black_height);
            let r = self.validade(&u.right.borrow(), *u.color.borrow(), black_height);
            (
                red_red + l.0 + r.0,
                std::cmp::min(l.1, r.1),
                std::cmp::max(l.2, r.2),
                left_leaning && l.3 && r.3,
            )
        } else {
            (0, black_height, black_height, true)
        }
    }
}

impl<T> SSet<T> for RedBlackTree<T>
where
    T: PartialOrd + Clone + Default,
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
            Some(n) if &*n.x.borrow() == x => {
                let mut u = Some(n);
                let mut w = u.as_ref().and_then(|u| u.right.borrow().clone());
                if w.is_none() {
                    w = u;
                    u = w.as_ref().and_then(|w| w.left.borrow().clone());
                } else {
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
                    u.as_ref().unwrap().x.swap(&w.as_ref().unwrap().x);
                    u = w.as_ref().and_then(|w| w.right.borrow().clone());
                }
                let p = w
                    .as_ref()
                    .and_then(|w| w.parent.borrow().as_ref().and_then(|p| p.upgrade()));
                let color = if let (Some(u), Some(w)) = (&u, &w) {
                    let c = *u.color.borrow() as isize + *w.color.borrow() as isize;
                    *u.color.borrow_mut() = match c {
                        0 => Color::Red,
                        1 => Color::Black,
                        _ => Color::WBlack,
                    };
                    c
                } else {
                    1 + *w.as_ref().unwrap().color.borrow() as isize
                };
                let res = self.splice(w.unwrap());
                self.remove_fixup(color, u, p);
                res
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
    use chapter04::skiplistsset::SkiplistSSet;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_redblacktree() {
        let mut redblacktree = RedBlackTree::<usize>::new();
        // add test
        redblacktree.add(1);
        redblacktree.add(2);
        redblacktree.add(3);
        redblacktree.add(4);
        redblacktree.add(5);
        redblacktree.add(6);
        redblacktree.add(7);
        assert!(!redblacktree.add(7));
        assert!(redblacktree.is_a_valid_red_black_tree());
        assert_eq!(redblacktree.size(), 7);
        // find test
        assert_eq!(redblacktree.find(&1), Some(1));
        assert_eq!(redblacktree.find(&3), Some(3));
        assert_eq!(redblacktree.find(&5), Some(5));
        assert_eq!(redblacktree.find(&7), Some(7));
        assert_eq!(redblacktree.find(&8), None);
        // remove test
        redblacktree.add(8);
        redblacktree.add(9);
        redblacktree.add(10);
        redblacktree.add(11);
        redblacktree.add(12);
        redblacktree.add(13);
        redblacktree.add(14);
        redblacktree.add(15);
        redblacktree.add(16);
        redblacktree.add(17);
        redblacktree.add(19);
        redblacktree.add(20);
        assert!(redblacktree.is_a_valid_red_black_tree());
        assert_eq!(redblacktree.remove(&1), Some(1));
        assert_eq!(redblacktree.remove(&3), Some(3));
        assert_eq!(redblacktree.remove(&2), Some(2));
        assert_eq!(redblacktree.remove(&6), Some(6));
        assert!(redblacktree.is_a_valid_red_black_tree());
        assert_eq!(redblacktree.remove(&5), Some(5));
        assert_eq!(redblacktree.remove(&20), Some(20));
        assert_eq!(redblacktree.remove(&9), Some(9));
        assert_eq!(redblacktree.remove(&12), Some(12));
        assert!(redblacktree.is_a_valid_red_black_tree());
        assert_eq!(redblacktree.remove(&11), Some(11));
        assert_eq!(redblacktree.remove(&18), None);
        assert_eq!(redblacktree.remove(&17), Some(17));
        assert_eq!(redblacktree.remove(&13), Some(13));
        assert_eq!(redblacktree.remove(&12), None);
        assert_eq!(redblacktree.remove(&4), Some(4));
        assert!(redblacktree.is_a_valid_red_black_tree());
        assert_eq!(redblacktree.remove(&7), Some(7));
        assert_eq!(redblacktree.remove(&16), Some(16));
        assert_eq!(redblacktree.remove(&14), Some(14));
        assert_eq!(redblacktree.remove(&19), Some(19));
        assert_eq!(redblacktree.remove(&8), Some(8));
        //println!("{:#?}", redblacktree);
        assert!(redblacktree.is_a_valid_red_black_tree());
        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut set: SkiplistSSet<i32> = SkiplistSSet::new();
        for _ in 0..5 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                set.add(x);
                assert!(redblacktree.is_a_valid_red_black_tree());
            }
            assert_eq!(redblacktree.size(), set.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = set.find(&x);
                let y2 = redblacktree.find(&x);
                assert_eq!(y1, y2);
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let b1 = set.remove(&x);
                let b2 = redblacktree.remove(&x);
                assert_eq!(b1, b2);
                assert!(redblacktree.is_a_valid_red_black_tree());
            }
            assert_eq!(redblacktree.size(), set.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = set.find(&x);
                let y2 = redblacktree.find(&x);
                assert_eq!(y1, y2);
            }
        }

        let n = 3;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut set: SkiplistSSet<i32> = SkiplistSSet::new();
        for x in 0..n {
            redblacktree.add(x);
            set.add(x);
            assert!(redblacktree.is_a_valid_red_black_tree());
        }
        assert_eq!(redblacktree.size(), set.size());
        for x in 0..n {
            let y1 = set.find(&x);
            let y2 = redblacktree.find(&x);
            assert_eq!(y1, y2);
        }
        for x in 0..2 {
            let b1 = set.remove(&x);
            let b2 = redblacktree.remove(&x);
            assert_eq!(b1, b2);
            assert!(redblacktree.is_a_valid_red_black_tree());
        }
        assert_eq!(redblacktree.size(), set.size());
        for x in 0..n {
            let y1 = set.find(&x);
            let y2 = redblacktree.find(&x);
            assert_eq!(y1, y2);
        }
        redblacktree.remove(&2);
        assert!(redblacktree.is_a_valid_red_black_tree());

        // test large linked list for stack overflow.
        let mut bst = RedBlackTree::<i32>::new();
        let num = 1000000;
        for i in 0..num {
            bst.add(i);
        }
        println!("fin");
    }
}
