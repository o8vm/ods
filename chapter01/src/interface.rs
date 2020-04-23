pub trait Queue<T> {
    fn add(&mut self, value: T);
    fn remove(&mut self) -> Option<T>;
}

pub trait Stack<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
}

pub trait List<T: Clone> {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<T>;
    fn set(&mut self, index: usize, value: T) -> Option<T>;
    fn add(&mut self, index: usize, value: T);
    fn remove(&mut self, index: usize) -> Option<T>;
}

pub trait USet<T: Eq> {
    fn size(&self) -> usize;
    fn add(&mut self, value: T) -> bool;
    fn remove(&mut self, value: &T) -> Option<T>;
    fn find(&self, value: &T) -> Option<&T>;
}

pub trait SSet<T: Ord + Clone> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    fn find(&self, x: T) -> Option<T>;
}
