use super::array_stack::Array as ArrayStack;
use chapter01::interface::List;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    blocks: ArrayStack<Rc<[RefCell<Option<T>>]>>,
    len: usize,
}

impl<T: Clone> Array<T> {
    pub fn new() -> Self {
        Self {
            blocks: ArrayStack::new(),
            len: 0,
        }
    }
    fn i2b(index: usize) -> usize {
        let db = (-3.0 + (9.0 + 8.0 * index as f64).sqrt()) / 2f64;
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
        while r > 0 && (r - 2) * (r - 1) / 2 >= self.len {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Option<T> {
        let b = Self::i2b(index);
        let j = index - b * (b + 1) / 2;
        match self.blocks.get(b).unwrap()[j].borrow().as_ref() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    fn set(&mut self, index: usize, value: T) -> Option<T> {
        let b = Self::i2b(index);
        let j = index - b * (b + 1) / 2;
        self.blocks.get(b).unwrap()[j].borrow_mut().replace(value)
    }

    fn add(&mut self, index: usize, value: T) {
        assert!(index <= self.len);
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.len + 1 {
            self.grow();
        }
        self.len += 1;
        for j in (index + 1..self.len).rev() {
            self.set(j, self.get(j - 1).unwrap());
        }
        self.set(index, value);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        assert!(index < self.len);
        let value = self.get(index);
        for j in index..self.len - 1 {
            self.set(j, self.get(j + 1).unwrap());
        }
        let eb = Self::i2b(self.len - 1);
        let ej = self.len - 1 - eb * (eb + 1) / 2;
        self.blocks.get(eb).unwrap()[ej].borrow_mut().take();
        self.len -= 1;
        let r = self.blocks.size();
        if (r - 2) * (r - 1) / 2 <= self.len {
            self.shrink();
        }
        value
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;
    #[test]
    fn test_rootish_array_stack() {
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
        println!("RootishArrayStack = {:?}", rootish_array_stack);
    }
}
