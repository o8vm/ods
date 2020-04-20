use chapter01::interface::List;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct DLList<T> {
    head: Link<T>,
    tail: Wink<T>,
    len: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Wink<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            prev: None,
        }))
    }
}

impl<T: Default> DLList<T> {
    pub fn new() -> Self {
        let dummy1: Rc<RefCell<Node<T>>> = Default::default();
        let dummy2: Rc<RefCell<Node<T>>> = Default::default();
        dummy1.borrow_mut().next = Some(dummy2.clone());
        dummy2.borrow_mut().prev = Some(Rc::downgrade(&dummy1));
        Self {
            head: Some(dummy1),
            tail: Some(Rc::downgrade(&dummy2)),
            len: 0,
        }
    }

    fn get_link(&self, index: usize) -> Link<T> {
        let mut p: Link<T>;
        if index < self.len / 2 {
            p = self
                .head
                .clone()
                .and_then(|dummy| dummy.borrow().next.clone());
            for _j in 0..index {
                p = p.and_then(|p| p.borrow().next.clone());
            }
        } else {
            p = self.tail.clone().and_then(|p| p.upgrade().clone());
            for _j in (index + 1..=self.len).rev() {
                p = p.and_then(|p| p.borrow().prev.clone().and_then(|p| p.upgrade().clone()));
            }
        }
        p
    }

    fn add_before(&mut self, w: Link<T>, value: T) {
        let u = Node::new(value);
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
        self.len += 1;
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
        self.len -= 1;
    }
}

impl<T: Clone + Default> List<T> for DLList<T> {
    fn size(&self) -> usize {
        self.len
    }
    fn get(&self, index: usize) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.get_link(index).map(|p| p.borrow().value.clone())
        }
    }
    fn set(&mut self, index: usize, value: T) -> Option<T> {
        if self.len > 0 {
            self.get_link(index).map(|p| {
                let ret = p.borrow().value.clone();
                p.borrow_mut().value = value;
                ret
            })
        } else {
            None
        }
    }
    fn add(&mut self, index: usize, value: T) {
        self.add_before(self.get_link(index), value);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let w = self.get_link(index);
        self.remove_link(w.clone());
        match w {
            Some(w) => Some(Rc::try_unwrap(w).ok().unwrap().into_inner().value),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::DLList;
    use chapter01::interface::List;
    #[test]
    fn test_dllist() {
        let mut dllist: DLList<char> = DLList::new();
        assert_eq!(dllist.size(), 0);
        dllist.add(0, 'a');
        dllist.add(1, 'b');
        dllist.add(2, 'c');
        dllist.add(3, 'd');
        dllist.add(4, 'e');
        assert_eq!(dllist.get(0), Some('a'));
        assert_eq!(dllist.get(1), Some('b'));
        assert_eq!(dllist.get(2), Some('c'));
        assert_eq!(dllist.get(3), Some('d'));
        assert_eq!(dllist.get(4), Some('e'));
        assert_eq!(dllist.set(1, 'x'), Some('b'));
        assert_eq!(dllist.get(1), Some('x'));
        assert_eq!(dllist.remove(2), Some('c'));
        dllist.add(2, 'y');
        assert_eq!(dllist.get(2), Some('y'));
        println!("\nDLList = {:?}\n", dllist);
        for elem in "axyde".chars() {
            assert_eq!(dllist.remove(0), Some(elem));
        }
        assert_eq!(dllist.remove(0), None);
        assert_eq!(dllist.get(0), None);
    }
}
