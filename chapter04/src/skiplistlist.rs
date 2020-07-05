#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop)]
use chapter01::interface::List;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SkiplistList<T: Clone + Default> {
    head: Link<T>,
    h: usize,
    n: usize,
}

impl<T> Drop for SkiplistList<T>
where
    T: Clone + Default,
{
    fn drop(&mut self) {
        while self.remove(0).is_some() {}
    }
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    x: T,
    length: Vec<usize>,
    next: Vec<Link<T>>,
}

impl<T> Node<T> {
    fn new(x: T, h: usize) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            x,
            length: vec![0; h + 1],
            next: vec![None; h + 1],
        }))
    }
}

impl<T: Default + Clone> SkiplistList<T> {
    pub fn new() -> Self {
        let sentinel = Node::new(Default::default(), 32);
        Self {
            head: Some(sentinel),
            h: 0,
            n: 0,
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
    fn find_pred_node(&self, i: usize) -> Link<T> {
        match self.head {
            Some(ref sentinel) => {
                let mut n = Rc::clone(sentinel);
                let mut j = 0;
                for r in (0..=self.h).rev() {
                    loop {
                        let u = Rc::clone(&n);
                        match u.borrow().next[r] {
                            Some(ref u) if j + n.borrow().length[r] - 1 < i => {
                                j += n.borrow().length[r];
                                n = Rc::clone(u)
                            }
                            _ => break,
                        };
                    }
                }
                Some(n)
            }
            None => None,
        }
    }
    fn add_node(&mut self, i: usize, w: Rc<RefCell<Node<T>>>) {
        if let Some(ref sentinel) = self.head {
            let mut n = Rc::clone(sentinel);
            let mut j = 0;
            for r in (0..=self.h).rev() {
                loop {
                    let u = Rc::clone(&n);
                    match u.borrow().next[r] {
                        Some(ref u) if j + n.borrow().length[r] - 1 < i => {
                            j += n.borrow().length[r];
                            n = Rc::clone(u);
                        }
                        _ => break,
                    };
                }
                n.borrow_mut().length[r] += 1;
                if r < w.borrow().next.len() {
                    let next = n.borrow_mut().next[r].take();
                    if let Some(u) = next {
                        w.borrow_mut().next[r] = Some(u);
                        w.borrow_mut().length[r] = n.borrow().length[r] + j - i - 1;
                    }
                    n.borrow_mut().next[r] = Some(Rc::clone(&w));
                    n.borrow_mut().length[r] = i + 1 - j;
                }
            }
            self.n += 1;
        }
    }
}

impl<T: Clone + Default> List<T> for SkiplistList<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> Option<T> {
        match self.find_pred_node(i) {
            Some(ref u) if u.borrow().next[0].is_some() => {
                u.borrow().next[0].as_ref().map(|u| u.borrow().x.clone())
            }
            _ => None,
        }
    }
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        match self.find_pred_node(i) {
            Some(ref u) if u.borrow().next[0].is_some() => u.borrow().next[0].as_ref().map(|u| {
                let y = u.borrow().x.clone();
                u.borrow_mut().x = x;
                y
            }),
            _ => None,
        }
    }
    fn add(&mut self, i: usize, x: T) {
        assert!(i <= self.size());
        let w = Node::new(x, Self::pick_height());
        if w.borrow().next.len() - 1 > self.h {
            if let Some(sentinel) = self
                .head
                .as_ref()
                .filter(|sentinel| sentinel.borrow().next.len() < w.borrow().next.len())
            {
                let height = sentinel.borrow().next.len();
                sentinel
                    .borrow_mut()
                    .next
                    .extend_from_slice(&vec![None; w.borrow().next.len() - height]);
                sentinel.borrow_mut().length.extend_from_slice(&vec![
                    0;
                    w.borrow().length.len()
                        - height
                ]);
            }
            self.h = w.borrow().next.len() - 1;
        }
        self.add_node(i, w);
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        match self.head {
            Some(ref sentinel) => {
                let mut n = Rc::clone(sentinel);
                let mut del = None;
                let rh = self.h;
                let mut j = 0;
                for r in (0..=rh).rev() {
                    let removed = loop {
                        let u = Rc::clone(&n);
                        match u.borrow().next[r] {
                            Some(ref u) if j + n.borrow().length[r] - 1 < i => {
                                j += n.borrow().length[r];
                                n = Rc::clone(u);
                            }
                            Some(ref _u) if j + n.borrow().length[r] - 1 == i => {
                                break true;
                            }
                            _ => break false,
                        };
                    };
                    if n.borrow().length[r] > 0 {
                        n.borrow_mut().length[r] -= 1;
                    }
                    if removed {
                        del = n.borrow_mut().next[r].take();
                        if let Some(del) = del.as_ref() {
                            let length = del.borrow().length[r];
                            if let Some(next) = del.borrow_mut().next[r].take() {
                                n.borrow_mut().next[r] = Some(next);
                                n.borrow_mut().length[r] += length;
                            } else if Rc::ptr_eq(&n, self.head.as_ref().unwrap()) {
                                if let Some(sentinel) = self.head.as_ref() {
                                    sentinel.borrow_mut().next.pop();
                                    sentinel.borrow_mut().length.pop();
                                }
                                if self.h > 0 {
                                    self.h -= 1;
                                }
                            }
                        }
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
}

#[cfg(test)]
mod test {
    use super::SkiplistList;
    use chapter01::interface::List;
    #[test]
    fn test_skiplistlist() {
        let mut skiplistlist: SkiplistList<char> = SkiplistList::new();
        skiplistlist.add(0, '0');
        skiplistlist.add(1, '1');
        skiplistlist.add(2, '2');
        skiplistlist.add(3, '3');
        skiplistlist.add(4, '4');
        skiplistlist.add(5, '5');
        skiplistlist.add(6, '6');
        skiplistlist.add(4, 'x');
        skiplistlist.set(4, 'y');
        for (i, elem) in "012345678".chars().enumerate() {
            match i {
                0..=3 => assert_eq!(skiplistlist.get(i), Some(elem)),
                4 => {
                    assert_eq!(skiplistlist.get(i), Some('y'));
                    assert_eq!(skiplistlist.get(i + 1), Some(elem));
                }
                5..=6 => assert_eq!(skiplistlist.get(i + 1), Some(elem)),
                _ => break,
            }
        }
        let mut skiplistlist: SkiplistList<char> = SkiplistList::new();
        skiplistlist.add(0, '0');
        skiplistlist.add(1, '1');
        skiplistlist.add(2, '2');
        skiplistlist.add(3, '3');
        skiplistlist.add(4, '4');
        skiplistlist.add(5, '5');
        skiplistlist.add(6, '6');
        assert_eq!(skiplistlist.remove(3), Some('3'));
        assert_eq!(skiplistlist.get(0), Some('0'));
        assert_eq!(skiplistlist.get(1), Some('1'));
        assert_eq!(skiplistlist.get(2), Some('2'));
        assert_eq!(skiplistlist.get(3), Some('4'));
        assert_eq!(skiplistlist.get(4), Some('5'));
        assert_eq!(skiplistlist.get(5), Some('6'));

        // test large linked list for stack overflow.
        let mut skiplistlist: SkiplistList<u64> = SkiplistList::new();
        let num = 100000;
        for i in 0..num {
            skiplistlist.add(skiplistlist.size(), i);
        }
        println!("fin");
    }
}
