#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    buf: Box<[Option<T>]>,
    ndx: usize,
    len: usize,
}

impl<T> Array<T> {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn pos(&self) -> usize {
        self.ndx
    }
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Self::allocate_in_heap(capacity),
            ndx: 0,
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.buf[(self.ndx + index) % self.capacity()].as_ref()
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        self.buf[(self.ndx + index) % self.capacity()].replace(value)
    }

    pub fn add(&mut self, index: usize, value: T) {
        if self.len == self.capacity() {
            self.resize();
        }
        if index < self.len / 2 {
            self.ndx = if self.ndx == 0 {
                self.capacity() - 1
            } else {
                self.ndx - 1
            };
            if index > 0 {
                for k in 0..index - 1 {
                    self.buf[(self.ndx + k) % self.capacity()] =
                        self.buf[(self.ndx + k + 1) % self.capacity()].take();
                }
            }
        } else {
            for k in (index + 1..=self.len).rev() {
                self.buf[(self.ndx + k) % self.capacity()] =
                    self.buf[(self.ndx + k - 1) % self.capacity()].take();
            }
        }
        self.buf[(self.ndx + index) % self.capacity()] = Some(value);
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let value = self.buf[(self.ndx + index) % self.capacity()].take();
        if index < self.len / 2 {
            for k in (1..=index).rev() {
                self.buf[(self.ndx + k) % self.capacity()] =
                    self.buf[(self.ndx + k - 1) % self.capacity()].take();
            }
            self.ndx = (self.ndx + 1) % self.capacity();
        } else {
            for k in index..self.len - 1 {
                self.buf[(self.ndx + k) % self.capacity()] =
                    self.buf[(self.ndx + k + 1) % self.capacity()].take();
            }
        }
        self.len -= 1;
        if self.capacity() > 3 * self.len {
            self.resize();
        }
        value
    }

    pub fn resize(&mut self) {
        if self.capacity() == 0 {
            self.buf = Self::allocate_in_heap(1);
        } else {
            let new_buf = Self::allocate_in_heap(self.len * 2);
            let mut old_buf = std::mem::replace(&mut self.buf, new_buf);
            for k in 0..self.len {
                self.buf[k] = old_buf[(self.ndx + k) % old_buf.len()].take();
            }
        }
        self.ndx = 0;
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    #[test]
    fn arraydeque_add_remove() {
        let mut array_deque: Array<char> = Array::new();
        assert_eq!(array_deque.len(), 0);
        array_deque.add(0, 'A');
        array_deque.add(0, 'B');
        array_deque.add(0, 'C');
        array_deque.add(0, 'D');
        array_deque.add(0, 'E');
        assert_eq!(array_deque.remove(array_deque.len - 1), Some('A'));
        assert_eq!(array_deque.remove(array_deque.len - 1), Some('B'));
        assert_eq!(array_deque.remove(array_deque.len - 1), Some('C'));
        assert_eq!(array_deque.remove(array_deque.len - 1), Some('D'));
        assert_eq!(array_deque.remove(array_deque.len - 1), Some('E'));
        array_deque.add(0, 'A');
        array_deque.add(0, 'B');
        array_deque.add(0, 'C');
        array_deque.add(0, 'D');
        array_deque.add(0, 'E');
        array_deque.add(0, 'F');
        assert_eq!(array_deque.remove(3), Some('C'));
        array_deque.add(3, 'X');
        assert_eq!(array_deque.remove(5), Some('A'));
    }

    #[test]
    fn arraydeque_set_get() {
        let mut array_deque: Array<char> = Array::new();
        assert_eq!(array_deque.len(), 0);
        array_deque.add(0, 'A');
        array_deque.add(0, 'B');
        array_deque.add(0, 'C');
        array_deque.add(0, 'D');
        array_deque.add(0, 'E');
        array_deque.add(0, 'F');
        array_deque.set(1, 'e');
        array_deque.set(5, 'a');
        assert_eq!(array_deque.get(2), Some(&'D'));
        assert_eq!(array_deque.get(1), Some(&'e'));
        assert_eq!(array_deque.get(array_deque.pos() - 1), Some(&'a'));
    }
}
