use super::array_stack::Array as ArrayStack;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: std::fmt::Debug> Array<T> {
    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.front.len() {
            self.front.get(self.front.len() - index - 1)
        } else {
            self.back.get(index - self.front.len())
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < self.front.len() {
            self.front.set(self.front.len() - index - 1, value)
        } else {
            self.back.set(index - self.front.len(), value)
        }
    }

    pub fn add(&mut self, index: usize, value: T) {
        if index < self.front.len() {
            self.front.add(self.front.len() - index, value);
        } else {
            self.back.add(index - self.front.len(), value);
        }
        self.balance();
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let value;
        if index < self.front.len() {
            value = self.front.remove(self.front.len() - index - 1);
        } else {
            value = self.back.remove(index - self.front.len());
        }
        self.balance();
        value
    }

    pub fn balance(&mut self) {
        if 3 * self.front.len() < self.back.len() || 3 * self.back.len() < self.front.len() {
            let flen = self.front.len();
            let n = flen + self.back.len();
            let nf = n / 2;
            let nb = n - nf;
            let mut af: ArrayStack<T> = ArrayStack::with_capacity(std::cmp::max(2 * nf, 1));
            let mut bf: ArrayStack<T> = ArrayStack::with_capacity(std::cmp::max(2 * nb, 1));
            for i in 0..nf {
                af.add(nf - i - 1, self.remove(0).unwrap());
            }
            for i in 0..nb {
                bf.add(i, self.remove(0).unwrap());
            }
            std::mem::replace(&mut self.front, af);
            std::mem::replace(&mut self.back, bf);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    #[test]
    fn dualarraydeque_add_remove() {
        let mut dual_array_deque: Array<char> = Array::new();
        assert_eq!(dual_array_deque.len(), 0);
        dual_array_deque.add(0, 'A');
        dual_array_deque.add(1, 'B');
        dual_array_deque.add(2, 'C');
        dual_array_deque.add(3, 'D');
        assert_eq!(dual_array_deque.get(0), Some(&'A'));
        assert_eq!(dual_array_deque.get(1), Some(&'B'));
        assert_eq!(dual_array_deque.get(2), Some(&'C'));
        assert_eq!(dual_array_deque.get(3), Some(&'D'));
        dual_array_deque.add(3, 'x');
        println!("{:?}", dual_array_deque);
        dual_array_deque.add(4, 'y');
        println!("{:?}", dual_array_deque);
        assert_eq!(dual_array_deque.remove(0), Some('A'));
        println!("{:?}", dual_array_deque);
        assert_eq!(dual_array_deque.get(0), Some(&'B'));
        assert_eq!(dual_array_deque.get(1), Some(&'C'));
        assert_eq!(dual_array_deque.get(2), Some(&'x'));
        assert_eq!(dual_array_deque.get(3), Some(&'y'));
        assert_eq!(dual_array_deque.get(4), Some(&'D'));
    }
}
