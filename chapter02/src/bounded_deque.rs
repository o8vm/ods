use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    buf: Box<[Option<T>]>,
    ddx: usize,
    len: usize,
}

impl<T> Array<T> {
    pub fn pos(&self) -> usize {
        self.ddx
    }
    pub fn length(&self) -> usize {
        self.buf.len()
    }

    pub fn new(b: usize) -> Self {
        Self {
            buf: Self::allocate_in_heap(b + 1),
            ddx: 0,
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            match self.buf[(self.ddx + index) % self.length()] {
                Some(ref value) => Some(value.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < self.len {
            self.buf[(self.ddx + index) % self.length()].replace(value)
        } else {
            None
        }
    }

    fn add(&mut self, index: usize, value: T) {
        assert!(index <= self.len);
        assert_ne!(self.length(), self.len);
        if index < self.len / 2 {
            self.ddx = if self.ddx == 0 {
                self.length() - 1
            } else {
                self.ddx - 1
            };
            if index > 0 {
                for k in 0..index - 1 {
                    self.buf[(self.ddx + k) % self.length()] =
                        self.buf[(self.ddx + k + 1) % self.length()].take();
                }
            }
        } else {
            for k in (index + 1..=self.len).rev() {
                self.buf[(self.ddx + k) % self.length()] =
                    self.buf[(self.ddx + k - 1) % self.length()].take();
            }
        }
        self.buf[(self.ddx + index) % self.length()] = Some(value);
        self.len += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        assert!(index < self.len);
        let value = self.buf[(self.ddx + index) % self.length()].take();
        if index < self.len / 2 {
            for k in (1..=index).rev() {
                self.buf[(self.ddx + k) % self.length()] =
                    self.buf[(self.ddx + k - 1) % self.length()].take();
            }
            self.ddx = (self.ddx + 1) % self.length();
        } else {
            for k in index..self.len - 1 {
                self.buf[(self.ddx + k) % self.length()] =
                    self.buf[(self.ddx + k + 1) % self.length()].take();
            }
        }
        self.len -= 1;
        value
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;
    #[test]
    fn test_bounded_deque() {
        let mut bounded_deque: Array<char> = Array::new(5);
        bounded_deque.add(0, 'a');
        bounded_deque.add(1, 'b');
        bounded_deque.add(2, 'c');
        bounded_deque.add(3, 'd');
        for (i, elem) in "abcd".chars().enumerate() {
            assert_eq!(bounded_deque.get(i), Some(elem));
        }
        bounded_deque.add(3, 'x');
        bounded_deque.add(4, 'y');
        assert_eq!(bounded_deque.remove(0), Some('a'));
        bounded_deque.set(3, 'z');
        for (i, elem) in "bcxzd".chars().enumerate() {
            assert_eq!(bounded_deque.get(i), Some(elem));
        }
        for _i in 0..5 {
            bounded_deque.remove(0);
        }
        println!("BDeque = {:?}", bounded_deque);
    }
}
