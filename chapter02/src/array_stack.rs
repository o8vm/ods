use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    buf: Box<[Option<T>]>,
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
        let old_buf = std::mem::replace(&mut self.buf, new_buf);
        for (i, elem) in old_buf.into_vec().into_iter().enumerate().take(self.len) {
            self.buf[i] = elem;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            match self.buf[index] {
                Some(ref value) => Some(value.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < self.len {
            self.buf[index].replace(value)
        } else {
            None
        }
    }

    fn add(&mut self, index: usize, value: T) {
        if self.len + 1 >= self.length() {
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

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.len {
            let value = self.buf[index].take();
            self.buf[index..self.len].rotate_left(1);
            self.len -= 1;
            if self.length() >= 3 * self.len {
                self.resize();
            }
            value
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;

    #[test]
    fn test_array_stack() {
        let mut array_stack: Array<char> = Array::new();
        assert_eq!(array_stack.size(), 0);
        for (i, elem) in "bred".chars().enumerate() {
            array_stack.add(i, elem);
        }
        array_stack.add(2, 'e');
        array_stack.add(5, 'r');
        assert_eq!((array_stack.size(), array_stack.length()), (6, 10));
        for (i, elem) in "breedr".chars().enumerate() {
            assert_eq!(array_stack.get(i), Some(elem));
        }
        array_stack.add(5, 'e');
        array_stack.remove(4);
        array_stack.remove(4);
        assert_eq!((array_stack.size(), array_stack.length()), (5, 10));
        array_stack.remove(4);
        array_stack.remove(3);
        array_stack.set(2, 'i');
        assert_eq!((array_stack.size(), array_stack.length()), (3, 6));
        for (i, elem) in "bri".chars().enumerate() {
            assert_eq!(array_stack.get(i), Some(elem));
        }
        println!("ArrayStack = {:?}", array_stack);
    }
}
