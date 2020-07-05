use super::arraystack::Array as ArrayStack;
use chapter01::interface::List;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    blocks: ArrayStack<Rc<[RefCell<Option<T>>]>>,
    n: usize,
}

impl<T: Clone> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Array<T> {
    pub fn new() -> Self {
        Self {
            blocks: ArrayStack::new(),
            n: 0,
        }
    }
    fn i2b(i: usize) -> usize {
        let db = (-3.0 + (9.0 + 8.0 * i as f64).sqrt()) / 2f64;
        db.ceil() as usize
    }
    fn grow(&mut self) {
        let block = std::iter::repeat_with(Default::default)
            .take(self.blocks.size() + 1)
            .collect::<Rc<_>>();
        self.blocks.add(self.blocks.size(), block);
    }
    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        while r > 0 && (std::cmp::max(2, r) - 2) * (r - 1) / 2 >= self.n {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        let b = Self::i2b(i);
        let j = i - b * (b + 1) / 2;
        match self.blocks.get(b)?[j].borrow().as_ref() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let b = Self::i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get(b)?[j].borrow_mut().replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        assert!(i <= self.n);
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.n + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i + 1..self.n).rev() {
            self.set(j, self.get(j - 1).unwrap());
        }
        self.set(i, x);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i < self.n {
            let x = self.get(i);
            for j in i..self.n - 1 {
                self.set(j, self.get(j + 1).unwrap());
            }
            let eb = Self::i2b(self.n - 1);
            let ej = self.n - 1 - eb * (eb + 1) / 2;
            self.blocks.get(eb)?[ej].borrow_mut().take();
            self.n -= 1;
            let r = self.blocks.size();
            if (r - 2) * (r - 1) / 2 <= self.n {
                self.shrink();
            }
            x
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
    fn test_rootisharraystack() {
        let mut rootish_array_stack: Array<char> = Array::new();
        assert_eq!(rootish_array_stack.size(), 0);
        for (i, elem) in "abcdefgh".chars().enumerate() {
            rootish_array_stack.add(i, elem);
        }
        for (i, elem) in "abcdefgh".chars().enumerate() {
            assert_eq!(rootish_array_stack.get(i), Some(elem));
        }
        rootish_array_stack.add(2, 'x');
        rootish_array_stack.remove(1);
        rootish_array_stack.remove(7);
        rootish_array_stack.remove(6);
        for (i, elem) in "axcdef".chars().enumerate() {
            assert_eq!(rootish_array_stack.get(i), Some(elem));
        }
        println!("\nRootishArrayStack = {:?}\n", rootish_array_stack);
        let mut rootish_array_stack: Array<i32> = Array::new();
        println!("{:?}", rootish_array_stack);
        let num = 10;
        for i in 0..num {
            rootish_array_stack.add(rootish_array_stack.size(), i);
        }
        while rootish_array_stack.remove(0).is_some() {}
        println!("\nRootishArrayStack = {:?}\n", rootish_array_stack);
    }
}
