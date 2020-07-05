#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop)]
use chapter01::interface::List;
use chapter02::arraystack::Array as ArrayStack;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockStore<T: Clone> {
    blocks: ArrayStack<T>,
    free: ArrayStack<usize>,
}

impl<T: Clone> BlockStore<T> {
    pub fn new() -> Self {
        Self {
            blocks: ArrayStack::new(),
            free: ArrayStack::new(),
        }
    }
    pub fn place_block(&mut self, block: T) -> usize {
        if self.free.size() > 0 {
            self.free.remove(self.free.size() - 1).unwrap()
        } else {
            let i = self.blocks.size();
            self.blocks.add(i, block);
            i
        }
    }
    pub fn free_block(&mut self, i: usize) {
        self.blocks.take(i);
        self.free.add(self.free.size(), i);
    }
    pub fn read_block(&self, i: usize) -> Option<T> {
        self.blocks.get(i)
    }
    pub fn write_block(&mut self, i: usize, block: T) {
        self.blocks.set(i, block);
    }
}
