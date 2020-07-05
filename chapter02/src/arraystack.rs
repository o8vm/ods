use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
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
        let old_a = std::mem::replace(&mut self.a, new_a);
        for (i, elem) in old_a.into_vec().into_iter().enumerate().take(self.n) {
            self.a[i] = elem;
        }
    }
}

impl<T: PartialEq> Array<T> {
    pub fn contains(&self, j: T) -> bool {
        for i in 0..self.n {
            if self.a.get(i).unwrap().as_ref() == Some(&j) {
                return true;
            }
        }
        false
    }
}

impl<T: Clone> Array<T> {
    pub fn take(&mut self, i: usize) -> Option<T> {
        self.a.get_mut(i)?.take()
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        self.a.get(i)?.as_ref().cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut(i)?.replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.n + 1 >= self.length() {
            self.resize();
        }

        if i >= self.n {
            self.a[self.n] = Some(x);
        } else {
            self.a[i..self.n].rotate_right(1);
            let end = self.a[i].replace(x);
            self.a[self.n] = end;
        }
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut(i)?.take();
        if i < self.n {
            self.a[i..self.n].rotate_left(1);
            self.n -= 1;
            if self.length() >= 3 * self.n {
                self.resize();
            }
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;

    #[test]
    fn test_arraystack() {
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
        assert_eq!(array_stack.get(4), None);
        println!("\nArrayStack = {:?}\n", array_stack);
        let mut array_stack: Array<i32> = Array::new();
        let num = 10;
        for i in 0..num {
            array_stack.add(array_stack.size(), i);
        }
        while array_stack.remove(0).is_some() {}
        println!("\nArrayStack = {:?}\n", array_stack);
    }
}
