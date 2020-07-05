use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> Array<T> {
    pub fn pos(&self) -> usize {
        self.j
    }
    pub fn length(&self) -> usize {
        self.a.len()
    }

    pub fn new() -> Self {
        Self::with_length(1)
    }

    pub fn with_length(length: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(length),
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

    pub fn resize(&mut self) {
        let new_a = Self::allocate_in_heap(std::cmp::max(1, self.n * 2));
        let mut old_a = std::mem::replace(&mut self.a, new_a);
        for k in 0..self.n {
            self.a[k] = old_a[(self.j + k) % old_a.len()].take();
        }
        self.j = 0;
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        self.a.get((self.j + i) % self.length())?.as_ref().cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut((self.j + i) % self.length())?.replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        assert!(i <= self.n);
        if self.n + 1 >= self.length() {
            self.resize();
        }

        if i < self.n / 2 {
            self.j = if self.j == 0 {
                self.length() - 1
            } else {
                self.j - 1
            };
            for k in 0..i {
                self.a[(self.j + k) % self.length()] =
                    self.a[(self.j + k + 1) % self.length()].take();
            }
        } else {
            for k in (i + 1..=self.n).rev() {
                self.a[(self.j + k) % self.length()] =
                    self.a[(self.j + k - 1) % self.length()].take();
            }
        }
        self.a[(self.j + i) % self.length()] = Some(x);
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.n {
            None
        } else {
            let x = self.a[(self.j + i) % self.length()].take();
            if i < self.n / 2 {
                for k in (1..=i).rev() {
                    self.a[(self.j + k) % self.length()] =
                        self.a[(self.j + k - 1) % self.length()].take();
                }
                self.j = (self.j + 1) % self.length();
            } else {
                for k in i..self.n - 1 {
                    self.a[(self.j + k) % self.length()] =
                        self.a[(self.j + k + 1) % self.length()].take();
                }
            }
            self.n -= 1;
            if self.length() > 3 * self.n {
                self.resize();
            }
            x
        }
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;
    #[test]
    fn test_arraydeque() {
        let mut array_deque: Array<char> = Array::new();
        assert_eq!(array_deque.size(), 0);
        array_deque.get(0);
        array_deque.add(0, 'a');
        array_deque.add(1, 'b');
        array_deque.add(2, 'c');
        array_deque.add(3, 'd');
        for (i, elem) in "abcd".chars().enumerate() {
            assert_eq!(array_deque.get(i), Some(elem));
        }
        array_deque.add(3, 'x');
        array_deque.add(4, 'y');
        assert_eq!(array_deque.remove(0), Some('a'));
        array_deque.set(3, 'z');
        for (i, elem) in "bcxzd".chars().enumerate() {
            assert_eq!(array_deque.get(i), Some(elem));
        }
        println!("\nArrayDeque = {:?}\n", array_deque);
        let mut array_deque: Array<i32> = Array::new();
        let num = 10;
        for i in 0..num {
            array_deque.add(array_deque.size(), i);
        }
        while array_deque.remove(0).is_some() {}
        println!("\nArrayDeque = {:?}\n", array_deque);
    }
}
