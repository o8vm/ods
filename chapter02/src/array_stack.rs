#[derive(Debug, Clone, PartialEq, Default)]
pub struct Array<T> {
    buf: Box<[Option<T>]>,
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
            len: 0,
        }
    }

    pub fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            self.buf[index].as_ref()
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < self.len {
            self.buf[index].replace(value)
        } else {
            None
        }
    }

    pub fn add(&mut self, index: usize, value: T) {
        if self.len == self.capacity() {
            self.resize();
        }

        if index >= self.len {
            self.buf[self.len] = Some(value);
        } else {
            self.buf[index..self.len].rotate_right(1);
            let end = self.buf[index].replace(value);
            self.buf[self.len] = end;
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let value = self.buf[index].take();
        self.buf[index..self.len].rotate_left(1);
        self.len -= 1;
        if self.capacity() >= 3 * self.len() {
            self.resize();
        }
        value
    }

    pub fn resize(&mut self) {
        if self.capacity() == 0 {
            self.buf = Self::allocate_in_heap(1);
        } else {
            let n = self.len();
            let new_buf = Self::allocate_in_heap(n * 2);
            let old_buf = std::mem::replace(&mut self.buf, new_buf);

            for (i, elem) in old_buf.into_vec().into_iter().enumerate().take(n) {
                self.buf[i] = elem;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    #[test]
    fn arraystack_add_remove() {
        let mut array_stack: Array<usize> = Array::new();
        assert_eq!(array_stack.len(), 0);
        array_stack.add(0, 1);
        array_stack.add(1, 2);
        array_stack.add(2, 3);
        array_stack.add(1, 4);
        assert_eq!(array_stack.len(), 4);
        assert_eq!(array_stack.remove(3), Some(3));
        assert_eq!(array_stack.remove(1), Some(4));
        assert_eq!(array_stack.remove(1), Some(2));
        assert_eq!(array_stack.remove(0), Some(1));
        assert_eq!(array_stack.len(), 0);
    }

    #[test]
    fn arraystack_set_get() {
        let mut array_stack: Array<usize> = Array::new();
        assert_eq!(array_stack.len(), 0);
        array_stack.add(0, 1);
        array_stack.add(1, 2);
        array_stack.add(2, 3);
        array_stack.add(1, 4);
        assert_eq!(array_stack.len(), 4);
        assert_eq!(array_stack.get(0), Some(&1));
        assert_eq!(array_stack.get(3), Some(&3));
        array_stack.set(1, 5);
        array_stack.set(2, 6);
        assert_eq!(array_stack.get(1), Some(&5));
        assert_eq!(array_stack.get(2), Some(&6));
    }
}
