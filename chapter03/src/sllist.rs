use chapter01::interface::{Queue, Stack};
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SLList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
    }
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> Stack<T> for SLList<T> {
    fn push(&mut self, value: T) {
        let new = Node::new(value);
        match self.head.take() {
            Some(old) => new.borrow_mut().next = Some(old.clone()),
            None => self.tail = Some(new.clone()),
        }
        self.len += 1;
        self.head = Some(new);
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old| {
            if let Some(next) = old.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.len -= 1;
            Rc::try_unwrap(old).ok().unwrap().into_inner().value
        })
    }
}

impl<T> Queue<T> for SLList<T> {
    fn add(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.len += 1;
        self.tail = Some(new);
    }
    fn remove(&mut self) -> Option<T> {
        self.head.take().map(|old| {
            if let Some(next) = old.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.len -= 1;
            Rc::try_unwrap(old).ok().unwrap().into_inner().value
        })
    }
}

#[cfg(test)]
mod test {
    use super::SLList;
    use chapter01::interface::{Stack, Queue};
    #[test]
    fn test_sllist() {
        let mut sllist: SLList<char> = SLList::new();
        for elem in "abcde".chars() {
            sllist.add(elem);
        }
        sllist.add('x');
        assert_eq!(sllist.remove(), Some('a'));
        assert_eq!(sllist.pop(), Some('b'));
        sllist.push('y');
        for elem in "ycdex".chars() {
            assert_eq!(sllist.remove(), Some(elem));
        }
        assert_eq!(sllist.pop(), None);
    }
}