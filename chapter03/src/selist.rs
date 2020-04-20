use chapter01::interface::List;
use chapter02::bounded_deque::Array as BDeque;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;
type Loc<T> = (Link<T>, usize);

#[derive(Clone, Debug, Default)]
pub struct SEList<T> {
    head: Link<T>,
    tail: Wink<T>,
    len: usize,
    b: usize,
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
            len: 0,
            b,
        }
    }

    fn get_loc(&self, mut i: usize) -> Loc<T> {
        let mut p: Link<T>;
        if i < self.len / 2 {
            p = self
                .head
                .clone()
                .and_then(|dummy| dummy.borrow().next.clone());
            while i >= p.clone().map(|p| p.borrow().block.size()).unwrap() {
                i -= p.clone().map(|p| p.borrow().block.size()).unwrap();
                p = p.clone().and_then(|p| p.borrow().next.clone());
            }
            (p, i)
        } else {
            let mut idx = self.len;
            p = self.tail.clone().and_then(|p| p.upgrade().clone());
            while i < idx {
                p = p
                    .clone()
                    .and_then(|p| p.borrow().prev.clone().and_then(|p| p.upgrade().clone()));
                idx -= p.clone().map(|p| p.borrow().block.size()).unwrap();
            }
            (p, i - idx)
        }
    }

    fn add_before(&mut self, w: Link<T>) -> Link<T> {
        let u = Node::new(self.b);
        u.borrow_mut().prev = w.clone().and_then(|p| p.borrow().prev.clone());
        u.borrow_mut().next = w;
        u.borrow_mut()
            .next
            .clone()
            .map(|p| p.borrow_mut().prev = Some(Rc::downgrade(&u)));
        u.borrow_mut().prev.clone().and_then(|p| {
            p.upgrade()
                .clone()
                .map(|p| p.borrow_mut().next = Some(u.clone()))
        });
        Some(u)
    }

    fn remove_link(&mut self, w: Link<T>) {
        let prev = w.clone().and_then(|p| p.borrow_mut().prev.take());
        let next = w.and_then(|p| p.borrow_mut().next.take());
        prev.clone().and_then(|p| {
            p.upgrade()
                .clone()
                .map(|p| p.borrow_mut().next = next.clone())
        });
        next.map(|p| p.borrow_mut().prev = prev);
    }

    fn add_last(&mut self, value: T) {
        let mut last = self
            .tail
            .clone()
            .and_then(|p| p.upgrade().clone())
            .and_then(|p| p.borrow().prev.clone().and_then(|p| p.upgrade().clone()));
        if last.clone().and_then(|p| p.borrow().prev.clone()).is_none()
            || last.clone().map(|p| p.borrow().block.size()).unwrap() == self.b + 1
        {
            last = self.add_before(self.tail.clone().and_then(|p| p.upgrade()));
        }
        last.map(|p| {
            let size = p.borrow().block.size();
            p.borrow_mut().block.add(size, value);
        });
        self.len += 1;
    }

    fn spread(&mut self, u: Link<T>) {
        let mut w = u.clone();
        for _j in 0..self.b {
            w = w.clone().and_then(|p| p.borrow().next.clone());
        }
        w = self.add_before(w);
        while !Rc::ptr_eq(w.as_ref().unwrap(), u.as_ref().unwrap()) {
            while w.clone().map(|p| p.borrow().block.size()).unwrap() < self.b {
                w.clone().map(|p| {
                    let link = p.borrow().prev.clone().and_then(|p| p.upgrade().clone());
                    let size = link.clone().map(|p| p.borrow().block.size()).unwrap();
                    let x = link
                        .and_then(|p| p.borrow_mut().block.remove(size - 1))
                        .unwrap();
                    p.borrow_mut().block.add(0, x);
                });
            }
            w = w
                .and_then(|p| p.borrow().prev.clone())
                .and_then(|p| p.upgrade());
        }
    }
    fn gather(&mut self, u: Link<T>) {
        let mut w = u.clone();
        for _j in 0..self.b - 1 {
            while w.clone().map(|p| p.borrow().block.size()).unwrap() < self.b {
                w.clone().map(|p| {
                    let link = p.borrow().next.clone();
                    let size = p.borrow().block.size();
                    let x = link.and_then(|p| p.borrow_mut().block.remove(0)).unwrap();
                    p.borrow_mut().block.add(size, x);
                });
            }
            w = w.and_then(|p| p.borrow().next.clone());
        }
        self.remove_link(w);
    }
}

impl<T: Clone + Default> List<T> for SEList<T> {
    fn size(&self) -> usize {
        self.len
    }
    fn get(&self, index: usize) -> Option<T> {
        if self.len == 0 || index > self.len {
            None
        } else {
            let (p, j) = self.get_loc(index);
            p.and_then(|p| p.borrow().block.get(j))
        }
    }
    fn set(&mut self, index: usize, value: T) -> Option<T> {
        if self.len > 0 && index < self.len {
            let (p, j) = self.get_loc(index);
            p.and_then(|p| p.borrow_mut().block.set(j, value))
        } else {
            None
        }
    }
    fn add(&mut self, index: usize, value: T) {
        if index == self.len {
            self.add_last(value);
            return;
        }
        let (mut u, j) = self.get_loc(index);
        let v = u.clone();
        let mut r = 0;
        while r < self.b
            && u.clone().and_then(|p| p.borrow().next.clone()).is_some()
            && u.clone().and_then(|p| p.borrow().prev.clone()).is_some()
            && u.clone().map(|p| p.borrow().block.size()).unwrap() == self.b + 1
        {
            u = u.and_then(|p| p.borrow().next.clone());
            r += 1;
        }
        if r == self.b {
            self.spread(v.clone());
            u = v.clone();
        }
        if u.clone().and_then(|p| p.borrow().next.clone()).is_none() {
            u = self.add_before(u);
        }
        while !Rc::ptr_eq(u.as_ref().unwrap(), v.as_ref().unwrap()) {
            u.clone().map(|p| {
                let link = p.borrow().prev.clone().and_then(|p| p.upgrade().clone());
                let size = link.clone().map(|p| p.borrow().block.size()).unwrap();
                let x = link
                    .and_then(|p| p.borrow_mut().block.remove(size - 1))
                    .unwrap();
                p.borrow_mut().block.add(0, x);
            });
            u = u
                .and_then(|p| p.borrow().prev.clone())
                .and_then(|p| p.upgrade());
        }
        u.map(|p| p.borrow_mut().block.add(j, value));
        self.len += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        let (mut u, j) = self.get_loc(index);
        let v = u.clone();
        let _value = u.clone().and_then(|p| p.borrow().block.get(j));
        let mut r = 0;
        while r < self.b
            && u.clone().and_then(|p| p.borrow().next.clone()).is_some()
            && u.clone().and_then(|p| p.borrow().prev.clone()).is_some()
            && u.clone().map(|p| p.borrow().block.size()).unwrap() == self.b - 1
        {
            u = u.and_then(|p| p.borrow().next.clone());
            r += 1;
        }
        if r == self.b {
            self.gather(v.clone());
        }
        u = v.clone();
        let value = u.clone().and_then(|p| p.borrow_mut().block.remove(j));
        while u.clone().map(|p| p.borrow().block.size()).unwrap() < self.b - 1
            && u.clone()
                .and_then(|p| p.borrow().next.clone())
                .and_then(|p| p.borrow().next.clone())
                .is_some()
        {
            u.clone().map(|p| {
                let link = p.borrow().next.clone();
                let size = p.borrow().block.size();
                let x = link.and_then(|p| p.borrow_mut().block.remove(0)).unwrap();
                p.borrow_mut().block.add(size, x);
            });
            u = u.and_then(|p| p.borrow().next.clone());
        }
        if u.clone().map(|p| p.borrow().block.size()).unwrap() == 0 {
            self.remove_link(u);
        }
        self.len -= 1;
        value
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
    }
}
