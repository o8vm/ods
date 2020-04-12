pub trait Queue<T> {
    fn add(&mut self, value: T);
    fn remove(&mut self, index: usize) -> Option<T>;
}

pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
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

pub trait SSet<T: Ord> {
    fn size(&self) -> usize;
    fn add(&mut self, value: T) -> bool;
    fn remove(&mut self, value: &T) -> Option<T>;
    fn find(&self, value: &T) -> Option<&T>;
}