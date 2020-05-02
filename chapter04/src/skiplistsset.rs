use chapter01::interface::SSet;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SkiplistSSet<T: Ord> {
    head: Link<T>,
    stack: Vec<Link<T>>,
    h: usize,
    n: usize,
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T: Ord> {
    x: T,
    next: Vec<Link<T>>,
}

impl<T: Ord> Node<T> {
    fn new(x: T, h: usize) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            x,
            next: vec![None; h + 1],
        }))
    }
}

impl<T: Ord + Default> SkiplistSSet<T> {
    pub fn new(h: usize) -> Self {
        let sentinel = Node::new(Default::default(), h);
        Self {
            head: Some(sentinel),
            stack: vec![None; h + 1],
            h: 0,
            n: 0,
        }
    }

    fn find_pred_node(&self, x: T) -> Link<T> {
        match self.head {
            Some(ref sentinel) => {
                let mut n = Rc::clone(sentinel);
                for r in (0..=self.h).rev() {
                    loop {
                        let u = Rc::clone(&n);
                        match u.borrow().next[r] {
                            Some(ref u) if u.borrow().x < x => n = Rc::clone(u),
                            _ => break,
                        };
                    }
                }
                Some(n)
            }
            None => None,
        }
    }
    fn pick_height() -> usize {
        let z = rand::random::<usize>();
        let mut k = 0;
        let mut m = 1;
        while (z & m) != 0 {
            k += 1;
            m <<= 1;
        }
        k
    }
}

impl<T: Ord + Clone + Default> SSet<T> for SkiplistSSet<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn add(&mut self, x: T) -> bool {
        match self.head {
            Some(ref sentinel) => {
                let mut n = Rc::clone(sentinel);
                for r in (0..=self.h).rev() {
                    loop {
                        let u = Rc::clone(&n);
                        match u.borrow().next[r] {
                            Some(ref u) if u.borrow().x < x => n = Rc::clone(u),
                            Some(ref u) if u.borrow().x == x => return false,
                            _ => break,
                        };
                    }
                    self.stack[r] = Some(Rc::clone(&n));
                }
                let w = Node::new(x, Self::pick_height());
                while self.h < w.borrow().next.len() - 1 {
                    self.head
                        .as_ref()
                        .filter(|sentinel| sentinel.borrow().next.len() < w.borrow().next.len())
                        .map(|sentinel| {
                            sentinel.borrow_mut().next.push(None);
                        });
                    self.h += 1;
                    self.stack.push(Some(Rc::clone(sentinel)));
                }
                let height = w.borrow().next.len() - 1;
                for i in 0..=height {
                    match self.stack[i] {
                        Some(ref u) => {
                            w.borrow_mut().next[i] = u.borrow_mut().next[i].take();
                            u.borrow_mut().next[i] = Some(Rc::clone(&w));
                        }
                        None => break,
                    };
                }
                self.n += 1;
                true
            }
            None => false,
        }
    }

    fn remove(&mut self, x: T) -> Option<T> {
        match self.head {
            Some(ref sentinel) => {
                let mut n = Rc::clone(sentinel);
                let mut del = None;
                let rh = self.h;
                for r in (0..=rh).rev() {
                    let removed = loop {
                        let u = Rc::clone(&n);
                        match u.borrow().next[r] {
                            Some(ref u) if u.borrow().x < x => n = Rc::clone(u),
                            Some(ref u) if u.borrow().x == x => break true,
                            _ => break false,
                        };
                    };
                    if removed {
                        del = n.borrow_mut().next[r].take();
                        del.as_ref().map(|del| {
                            if let Some(next) = del.borrow_mut().next[r].take() {
                                n.borrow_mut().next[r] = Some(next);
                            } else {
                                if Rc::ptr_eq(&n, self.head.as_ref().unwrap()) {
                                    self.head
                                        .as_ref()
                                        .map(|sentinel| sentinel.borrow_mut().next.pop());
                                    self.stack.pop();
                                    self.h -= 1;
                                }
                            }
                        });
                    }
                }
                del.map(|del| {
                    self.n -= 1;
                    Rc::try_unwrap(del).ok().unwrap().into_inner().x
                })
            }
            None => None,
        }
    }

    fn find(&self, x: T) -> Option<T> {
        match self.find_pred_node(x) {
            Some(ref u) if u.borrow().next[0].is_some() => {
                u.borrow().next[0].as_ref().map(|u| u.borrow().x.clone())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::SkiplistSSet;
    use chapter01::interface::SSet;
    #[test]
    fn test_skiplistsset() {
        let mut skiplistsset: SkiplistSSet<u64> = SkiplistSSet::new(6);
        skiplistsset.add(0);
        skiplistsset.add(1);
        skiplistsset.add(2);
        skiplistsset.add(3);
        skiplistsset.add(5);
        skiplistsset.add(6);
        skiplistsset.add(7);
        for i in 0..8 {
            if i == 4 {
                continue;
            }
            assert_eq!(skiplistsset.find(i), Some(i));
        }
        assert_eq!(skiplistsset.size(), 7);
        skiplistsset.add(4);
        for i in 0..8 {
            assert_eq!(skiplistsset.find(i), Some(i));
        }
        assert_eq!(skiplistsset.remove(4), Some(4));
        for i in 0..8 {
            if i == 4 {
                continue;
            }
            assert_eq!(skiplistsset.find(i), Some(i));
        }
        assert_eq!(skiplistsset.remove(9), None);
        println!("\nSkiplistSSet = {:?}\n", skiplistsset);
    }
}
