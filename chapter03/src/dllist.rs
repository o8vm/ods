#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop)]
use chapter01::interface::List;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct DLList<T: Clone + Default> {
    head: Link<T>,
    tail: Wink<T>,
    n: usize,
}

impl<T> Drop for DLList<T>
where
    T: Clone + Default,
{
    fn drop(&mut self) {
        while self.remove(0).is_some() {}
    }
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    x: T,
    next: Link<T>,
    prev: Wink<T>,
}

impl<T> Node<T> {
    fn new(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            x,
            next: None,
            prev: None,
        }))
    }
}

impl<T: Default + Clone> DLList<T> {
    pub fn new() -> Self {
        let dummy1: Rc<RefCell<Node<T>>> = Default::default();
        let dummy2: Rc<RefCell<Node<T>>> = Default::default();
        dummy1.borrow_mut().next = Some(dummy2.clone());
        dummy2.borrow_mut().prev = Some(Rc::downgrade(&dummy1));
        Self {
            head: Some(dummy1),
            tail: Some(Rc::downgrade(&dummy2)),
            n: 0,
        }
    }

    fn get_link(&self, i: usize) -> Link<T> {
        let mut p: Link<T>;
        if i < self.n / 2 {
            p = self.head.as_ref().and_then(|d| d.borrow().next.clone());
            for _j in 0..i {
                p = p.and_then(|p| p.borrow().next.clone());
            }
        } else {
            p = self.tail.as_ref().and_then(|p| p.upgrade());
            for _j in (i + 1..=self.n).rev() {
                p = p.and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
            }
        }
        p
    }

    fn add_before(&mut self, w: Link<T>, x: T) {
        let u = Node::new(x);
        u.borrow_mut().prev = w.as_ref().and_then(|p| p.borrow().prev.clone());
        if let Some(p) = w.as_ref() {
            p.borrow_mut().prev = Some(Rc::downgrade(&u))
        }
        u.borrow_mut().next = w;
        u.borrow()
            .prev
            .as_ref()
            .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = Some(u.clone())));
        self.n += 1;
    }

    fn remove_link(&mut self, w: Link<T>) {
        let prev = w.as_ref().and_then(|p| p.borrow_mut().prev.take());
        let next = w.and_then(|p| p.borrow_mut().next.take());
        prev.as_ref()
            .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = next.clone()));
        if let Some(p) = next {
            p.borrow_mut().prev = prev
        }
        self.n -= 1;
    }
}

impl<T: Clone + Default> List<T> for DLList<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> Option<T> {
        if self.n == 0 {
            None
        } else {
            self.get_link(i).map(|p| p.borrow().x.clone())
        }
    }
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if self.n > 0 {
            self.get_link(i).map(|p| {
                let ret = p.borrow().x.clone();
                p.borrow_mut().x = x;
                ret
            })
        } else {
            None
        }
    }
    fn add(&mut self, i: usize, x: T) {
        self.add_before(self.get_link(i), x);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let w = self.get_link(i);
        self.remove_link(w.clone());
        match w {
            Some(w) => Some(Rc::try_unwrap(w).ok().unwrap().into_inner().x),
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

        // test large linked list for stack overflow.
        let mut dllist: DLList<i32> = DLList::new();
        let num = 10;
        for i in 0..num {
            dllist.add(dllist.size(), i);
        }
        while dllist.remove(0).is_some() {}
        let mut dllist: DLList<i32> = DLList::new();
        let num = 100000;
        for i in 0..num {
            dllist.add(dllist.size(), i);
        }
        println!("fin");
    }
}
