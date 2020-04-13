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
    
}