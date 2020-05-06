use chapter01::interface::Queue;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> Array<T> {
    pub fn length(&self) -> usize {
        self.a.len()
    }

    pub fn new() -> Self {
        Self::with_length(1)
    }

    pub fn with_length(capacity: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(capacity),
            j: 0,
            n: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_a = Self::allocate_in_heap(std::cmp::max(self.n * 2, 1));
        let mut old_a = std::mem::replace(&mut self.a, new_a);
        for k in 0..self.n {
            self.a[k] = old_a[(self.j + k) % old_a.len()].take();
        }
        self.j = 0;
    }
}

impl<T> Queue<T> for Array<T> {
    fn add(&mut self, value: T) {
        if self.n + 1 >= self.length() {
            self.resize();
        }
        self.a[(self.j + self.n) % self.length()] = Some(value);
        self.n += 1;
    }

    fn remove(&mut self) -> Option<T> {
        let x = self.a[self.j].take();
        self.j = (self.j + 1) % self.length();
        self.n -= 1;
        if self.length() >= 3 * self.n {
            self.resize();
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::Queue;
    #[test]
    fn test_arrayqueue() {
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
