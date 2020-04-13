use super::array_stack::Array as ArrayStack;
use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Clone> Array<T> {
    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }

    pub fn balance(&mut self) {
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let n = self.front.size() + self.back.size();
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

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, index: usize) -> Option<T> {
        if index < self.front.size() {
            self.front.get(self.front.size() - index - 1)
        } else {
            self.back.get(index - self.front.size())
        }
    }

    fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < self.front.size() {
            self.front.set(self.front.size() - index - 1, value)
        } else {
            self.back.set(index - self.front.size(), value)
        }
    }

    fn add(&mut self, index: usize, value: T) {
        if index < self.front.size() {
            self.front.add(self.front.size() - index, value);
        } else {
            self.back.add(index - self.front.size(), value);
        }
        self.balance();
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        let value;
        if index < self.front.size() {
            value = self.front.remove(self.front.size() - index - 1);
        } else {
            value = self.back.remove(index - self.front.size());
        }
        self.balance();
        value
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;
    #[test]
    fn test_dual_array_deque() {
        let mut dual_array_deque: Array<char> = Array::new();
        assert_eq!(dual_array_deque.size(), 0);
        dual_array_deque.add(0, 'A');
        dual_array_deque.add(1, 'B');
        dual_array_deque.add(2, 'C');
        dual_array_deque.add(3, 'D');
        assert_eq!(dual_array_deque.get(0), Some('A'));
        assert_eq!(dual_array_deque.get(1), Some('B'));
        assert_eq!(dual_array_deque.get(2), Some('C'));
        assert_eq!(dual_array_deque.get(3), Some('D'));
        dual_array_deque.add(3, 'x');
        dual_array_deque.add(4, 'y');
        assert_eq!(dual_array_deque.remove(0), Some('A'));
        assert_eq!(dual_array_deque.get(0), Some('B'));
        assert_eq!(dual_array_deque.get(1), Some('C'));
        assert_eq!(dual_array_deque.get(2), Some('x'));
        assert_eq!(dual_array_deque.get(3), Some('y'));
        assert_eq!(dual_array_deque.get(4), Some('D'));
        println!("DualArrayDeque = {:?}", dual_array_deque);
    }
}
