#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop)]
use chapter01::interface::List;
use chapter02::boundeddeque::Array as BDeque;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;
type Loc<T> = (Link<T>, usize);

#[derive(Clone, Debug, Default)]
pub struct SEList<T: Clone + Default> {
    head: Link<T>,
    tail: Wink<T>,
    n: usize,
    b: usize,
}

impl<T> Drop for SEList<T>
where
    T: Clone + Default,
{
    fn drop(&mut self) {
        while self.remove(0).is_some() {}
    }
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    block: BDeque<T>,
    next: Link<T>,
    prev: Wink<T>,
}

impl<T> Node<T> {
    fn new(b: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            block: BDeque::new(b + 1),
            next: None,
            prev: None,
        }))
    }
}

impl<T: Default + Clone> SEList<T> {
    pub fn new(b: usize) -> Self {
        let dummy1: Rc<RefCell<Node<T>>> = Default::default();
        let dummy2: Rc<RefCell<Node<T>>> = Default::default();
        dummy1.borrow_mut().next = Some(dummy2.clone());
        dummy2.borrow_mut().prev = Some(Rc::downgrade(&dummy1));
        Self {
            head: Some(dummy1),
            tail: Some(Rc::downgrade(&dummy2)),
            n: 0,
            b,
        }
    }

    fn get_loc(&self, mut i: usize) -> Loc<T> {
        let mut p: Link<T>;
        if i < self.n / 2 {
            p = self.head.as_ref().and_then(|d| d.borrow().next.clone());
            while i >= p.as_ref().map(|p| p.borrow().block.size()).unwrap() {
                i -= p.as_ref().map(|p| p.borrow().block.size()).unwrap();
                p = p.as_ref().and_then(|p| p.borrow().next.clone());
            }
            (p, i)
        } else {
            let mut idx = self.n;
            p = self.tail.as_ref().and_then(|p| p.upgrade());
            while i < idx {
                p = p
                    .as_ref()
                    .and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
                idx -= p.as_ref().map(|p| p.borrow().block.size()).unwrap();
            }
            (p, i - idx)
        }
    }

    fn add_before(&mut self, w: Link<T>) -> Link<T> {
        let u = Node::new(self.b);
        u.borrow_mut().prev = w.as_ref().and_then(|p| p.borrow().prev.clone());
        if let Some(p) = w.as_ref() {
            p.borrow_mut().prev = Some(Rc::downgrade(&u))
        }
        u.borrow_mut().next = w;
        u.borrow()
            .prev
            .as_ref()
            .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = Some(u.clone())));
        Some(u)
    }

    fn remove_link(&mut self, w: Link<T>) {
        let prev = w.as_ref().and_then(|p| p.borrow_mut().prev.take());
        let next = w.and_then(|p| p.borrow_mut().next.take());
        prev.as_ref()
            .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = next.clone()));
        if let Some(p) = next {
            p.borrow_mut().prev = prev
        }
    }

    fn add_last(&mut self, x: T) {
        let mut last = self
            .tail
            .as_ref()
            .and_then(|p| p.upgrade())
            .and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
        if let Some(ref p) = last {
            if p.borrow().prev.is_none() || p.borrow().block.size() == self.b + 1 {
                last = self.add_before(self.tail.as_ref().and_then(|p| p.upgrade()));
            }
            if let Some(p) = last {
                let s = p.borrow().block.size();
                p.borrow_mut().block.add(s, x);
                self.n += 1;
            }
        }
    }

    fn spread(&mut self, u: Link<T>) {
        let mut w = u.clone();
        for _j in 0..self.b {
            w = w.as_ref().and_then(|p| p.borrow().next.clone());
        }
        w = self.add_before(w);
        while !Rc::ptr_eq(w.as_ref().unwrap(), u.as_ref().unwrap()) {
            while w.as_ref().map(|p| p.borrow().block.size()).unwrap() < self.b {
                if let Some(p) = w.as_ref() {
                    let l = p.borrow().prev.as_ref().and_then(|p| p.upgrade());
                    let s = l.as_ref().map(|p| p.borrow().block.size()).unwrap();
                    let x = l.and_then(|p| p.borrow_mut().block.remove(s - 1)).unwrap();
                    p.borrow_mut().block.add(0, x);
                }
            }
            w = w.and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
        }
    }
    fn gather(&mut self, u: Link<T>) {
        let mut w = u;
        for _j in 0..self.b - 1 {
            while w.as_ref().map(|p| p.borrow().block.size()).unwrap() < self.b {
                if let Some(p) = w.as_ref() {
                    let l = p.borrow().next.clone();
                    let s = p.borrow().block.size();
                    let x = l.and_then(|p| p.borrow_mut().block.remove(0)).unwrap();
                    p.borrow_mut().block.add(s, x);
                }
            }
            w = w.and_then(|p| p.borrow().next.clone());
        }
        self.remove_link(w);
    }
}

impl<T: Clone + Default> List<T> for SEList<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, index: usize) -> Option<T> {
        if self.n == 0 || index > self.n {
            None
        } else {
            let (p, j) = self.get_loc(index);
            p.and_then(|p| p.borrow().block.get(j))
        }
    }
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if self.n > 0 && i < self.n {
            let (p, j) = self.get_loc(i);
            p.and_then(|p| p.borrow_mut().block.set(j, x))
        } else {
            None
        }
    }
    fn add(&mut self, i: usize, x: T) {
        if i == self.n {
            self.add_last(x);
            return;
        }
        let (mut u, j) = self.get_loc(i);
        let v = u.clone();
        let mut r = 0;
        while r < self.b
            && u.as_ref()
                .filter(|p| p.borrow().next.is_some() && p.borrow().prev.is_some())
                .is_some()
            && u.as_ref().map(|p| p.borrow().block.size()).unwrap() == self.b + 1
        {
            u = u.and_then(|p| p.borrow().next.clone());
            r += 1;
        }
        if r == self.b {
            self.spread(v.clone());
            u = v.clone();
        }
        if u.as_ref()
            .map(|p| p.borrow().next.is_none())
            .filter(|b| b == &true)
            .is_some()
        {
            u = self.add_before(u);
        }
        while !Rc::ptr_eq(u.as_ref().unwrap(), v.as_ref().unwrap()) {
            if let Some(p) = u.as_ref() {
                let l = p.borrow().prev.as_ref().and_then(|p| p.upgrade());
                let s = l.as_ref().map(|p| p.borrow().block.size()).unwrap();
                let x = l.and_then(|p| p.borrow_mut().block.remove(s - 1)).unwrap();
                p.borrow_mut().block.add(0, x);
            };
            u = u
                .and_then(|p| p.borrow().prev.clone())
                .and_then(|p| p.upgrade());
        }
        if let Some(p) = u {
            p.borrow_mut().block.add(j, x)
        }
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if self.n > 0 {
            let (mut u, j) = self.get_loc(i);
            let v = u.clone();
            let mut r = 0;
            while r < self.b
                && u.as_ref()
                    .filter(|p| p.borrow().next.is_some() && p.borrow().prev.is_some())
                    .is_some()
                && u.as_ref().map(|p| p.borrow().block.size()).unwrap() == self.b - 1
            {
                u = u.and_then(|p| p.borrow().next.clone());
                r += 1;
            }
            if r == self.b {
                self.gather(v.clone());
            }
            u = v;
            let x = u.as_ref().and_then(|p| p.borrow_mut().block.remove(j));
            while u.as_ref().map(|p| p.borrow().block.size()).unwrap() < self.b - 1
                && u.as_ref()
                    .and_then(|p| p.borrow().next.clone())
                    .and_then(|p| p.borrow().next.clone())
                    .is_some()
            {
                if let Some(p) = u.clone() {
                    let l = p.borrow().next.clone();
                    let s = p.borrow().block.size();
                    let x = l.and_then(|p| p.borrow_mut().block.remove(0)).unwrap();
                    p.borrow_mut().block.add(s, x);
                }
                u = u.and_then(|p| p.borrow().next.clone());
            }
            if u.as_ref().map(|p| p.borrow().block.size()).unwrap() == 0 {
                self.remove_link(u);
            }
            self.n -= 1;
            x
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::SEList;
    use chapter01::interface::List;
    #[test]
    fn test_selist() {
        let mut selist: SEList<char> = SEList::new(3);
        selist.add(0, 'a');
        selist.add(1, 'b');
        selist.add(2, 'c');
        selist.add(3, 'd');
        selist.add(4, 'e');
        selist.add(5, 'f');
        selist.add(6, 'g');
        selist.add(7, 'h');
        selist.add(8, 'i');
        selist.add(9, 'j');
        selist.add(1, 'x');
        assert_eq!(selist.size(), 11);
        for (i, elem) in "axbcdefghij".chars().enumerate() {
            assert_eq!(selist.get(i), Some(elem));
        }
        let mut selist: SEList<char> = SEList::new(3);
        selist.add(0, 'a');
        selist.add(1, 'b');
        selist.add(2, 'c');
        selist.add(3, 'd');
        selist.add(4, 'e');
        selist.add(5, 'f');
        selist.add(6, 'g');
        selist.add(7, 'h');
        selist.add(1, 'x');
        assert_eq!(selist.size(), 9);
        for (i, elem) in "axbcdefgh".chars().enumerate() {
            assert_eq!(selist.get(i), Some(elem));
        }
        let mut selist: SEList<char> = SEList::new(3);
        selist.add(0, 'a');
        selist.add(1, 'b');
        selist.add(2, 'c');
        selist.add(3, 'd');
        selist.add(4, 'e');
        selist.add(5, 'f');
        selist.add(6, 'g');
        selist.add(7, 'h');
        selist.add(8, 'i');
        selist.add(9, 'j');
        selist.add(10, 'k');
        selist.add(11, 'l');
        selist.add(1, 'x');
        assert_eq!(selist.size(), 13);
        for (i, elem) in "axbcdefghijkl".chars().enumerate() {
            assert_eq!(selist.get(i), Some(elem));
        }
        let mut selist: SEList<char> = SEList::new(3);
        selist.add(0, 'a');
        selist.add(1, 'b');
        selist.add(2, 'c');
        selist.add(3, 'd');
        selist.add(4, 'e');
        selist.add(5, 'f');
        selist.add(6, 'g');
        selist.add(7, 'h');
        selist.add(8, 'i');
        selist.add(9, 'j');
        selist.add(10, 'k');
        selist.add(11, 'l');
        selist.add(12, 'm');
        selist.add(13, 'n');
        selist.add(14, 'o');
        selist.add(15, 'p');
        assert_eq!(selist.size(), 16);
        assert_eq!(selist.remove(7), Some('h'));
        assert_eq!(selist.size(), 15);
        assert_eq!(selist.remove(6), Some('g'));
        assert_eq!(selist.size(), 14);
        assert_eq!(selist.remove(5), Some('f'));
        assert_eq!(selist.size(), 13);
        assert_eq!(selist.remove(1), Some('b'));
        assert_eq!(selist.size(), 12);
        assert_eq!(selist.remove(2), Some('d'));
        assert_eq!(selist.size(), 11);
        for (i, elem) in "aceijklmnop".chars().enumerate() {
            assert_eq!(selist.get(i), Some(elem));
        }
        //--
        assert_eq!(selist.remove(1), Some('c'));
        assert_eq!(selist.size(), 10);
        selist.remove(1);
        assert_eq!(selist.size(), 9);
        assert_eq!(selist.remove(4), Some('l'));
        assert_eq!(selist.remove(4), Some('m'));
        selist.add(4, 'x');
        assert_eq!(selist.remove(4), Some('x'));
        println!("\nSEList = {:?}\n", selist);
        let mut selist: SEList<i32> = SEList::new(3);
        let num = 10;
        for i in 0..num {
            selist.add(selist.size(), i);
        }
        while selist.remove(0).is_some() {}

        // test large linked list for stack overflow.
        let mut selist: SEList<i32> = SEList::new(3);
        let num = 100000;
        for i in 0..num {
            selist.add(selist.size(), i);
        }
        println!("fin");
    }
}
