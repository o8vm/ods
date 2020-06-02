use chapter01::interface::Queue;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinaryHeap<T> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T: Ord> BinaryHeap<T> {
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
    fn bubbleup(&mut self, mut i: usize) {
        let mut p = Self::parent(i);
        while self.a[i] < self.a[p] {
            self.a.swap(i, p);
            i = p;
            p = Self::parent(i);
        }
    }
    fn trickle_down(&mut self, i: usize) {

    }
    fn left(i: usize) -> usize {
        2 * i + 1
    }
    fn riht(i: usize) -> usize {
        2 * i + 2
    }
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }
}

impl<T: Ord> Queue<T> for BinaryHeap<T> {
    fn add(&mut self, x: T) {
        if self.n + 1 >= self.length() {
            self.resize();
        }
        self.a[self.n] = Some(x);
        self.n += 1;
        self.bubbleup(self.n - 1);
    }

    fn remove(&mut self) -> Option<T> {
        let x = self.a.get_mut(0)?.take();
        self.a[0] = self.a.get_mut(self.n - 1)?.take();
        self.n -= 1;
        self.trickle_down(0);
        if 3 * self.n < self.length() {
            self.resize();
        }
        x
    }
}