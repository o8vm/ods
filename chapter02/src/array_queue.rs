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

    pub fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn add(&mut self, value: T) {
        if self.len == self.capacity() {
            self.resize();
        }
        self.buf[(self.ndx + self.len) % self.capacity()] = Some(value);
        self.len += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        let value = self.buf[self.ndx].take();
        self.ndx = (self.ndx + 1) % self.capacity();
        self.len -= 1;
        if self.capacity() >= 3 * self.len {
            self.resize();
        }
        value
    }

    fn resize(&mut self) {
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
    fn arrayqueue_add_remove() {
        let mut array_queue: Array<char> = Array::new();
        assert_eq!(array_queue.len(), 0);
        array_queue.add('A');
        array_queue.add('B');
        assert_eq!(array_queue.remove(), Some('A'));
        array_queue.add('C');
        array_queue.add('D');
        assert_eq!(array_queue.remove(), Some('B'));
        array_queue.add('E');
        assert_eq!(array_queue.remove(), Some('C'));
        assert_eq!(array_queue.remove(), Some('D'));
        assert_eq!(array_queue.remove(), Some('E'));
    }
}
