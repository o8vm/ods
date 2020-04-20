use chapter01::interface::Queue;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    buf: Box<[Option<T>]>,
    ddx: usize,
    len: usize,
}

impl<T> Array<T> {
    pub fn length(&self) -> usize {
        self.buf.len()
    }

    pub fn new() -> Self {
        Self::with_length(0)
    }

    pub fn with_length(capacity: usize) -> Self {
        Self {
            buf: Self::allocate_in_heap(capacity),
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

    fn resize(&mut self) {
        let new_buf = Self::allocate_in_heap(std::cmp::max(self.len * 2, 1));
        let mut old_buf = std::mem::replace(&mut self.buf, new_buf);
        for k in 0..self.len {
            self.buf[k] = old_buf[(self.ddx + k) % old_buf.len()].take();
        }
        self.ddx = 0;
    }
}

impl<T> Queue<T> for Array<T> {
    fn add(&mut self, value: T) {
        if self.len + 1 >= self.length() {
            self.resize();
        }
        self.buf[(self.ddx + self.len) % self.length()] = Some(value);
        self.len += 1;
    }

    fn remove(&mut self) -> Option<T> {
        let value = self.buf[self.ddx].take();
        self.ddx = (self.ddx + 1) % self.length();
        self.len -= 1;
        if self.length() >= 3 * self.len {
            self.resize();
        }
        value
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::Queue;
    #[test]
    fn test_array_stack() {
        let mut array_queue: Array<char> = Array::new();
        for elem in "aaabc".chars() {
            array_queue.add(elem);
        }
        assert_eq!(array_queue.remove(), Some('a'));
        assert_eq!(array_queue.remove(), Some('a'));
        array_queue.add('d');
        array_queue.add('e');
        assert_eq!(array_queue.remove(), Some('a'));
        array_queue.add('f');
        array_queue.add('g');
        assert_eq!(array_queue.length(), 10);
        array_queue.add('h');
        assert_eq!(array_queue.remove(), Some('b'));
        println!("\nArrayQueue = {:?}\n", array_queue);
    }
}
