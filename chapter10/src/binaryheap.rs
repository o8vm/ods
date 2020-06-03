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
        while self.a.get(i) < self.a.get(p) {
            self.a.swap(i, p);
            i = p;
            p = Self::parent(i);
        }
    }
    fn trickle_down(&mut self, mut i: usize) {
        loop {
            let mut flag = false;
            let mut j = i;
            let r = Self::riht(i);
            if r < self.n && self.a.get(r) < self.a.get(i) {
                let l = Self::left(i);
                if self.a.get(l) < self.a.get(r) {
                    j = l;
                    flag = true;
                } else {
                    j = r;
                    flag = true;
                }
            } else {
                let l = Self::left(i);
                if l < self.n && self.a.get(l) < self.a.get(i) {
                    j = l;
                }
            }
            if flag == true {
                self.a.swap(i, j);
                i = j;
            } else {
                break;
            }
        }
    }
    fn left(i: usize) -> usize {
        2 * i + 1
    }
    fn riht(i: usize) -> usize {
        2 * i + 2
    }
    fn parent(i: usize) -> usize {
        (std::cmp::max(1, i) - 1) / 2
    }
}

impl<T: Ord> Queue<T> for BinaryHeap<T> {
    fn add(&mut self, x: T) {
        if self.n + 1 > self.length() {
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

#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::Queue;
    #[test]
    fn test_binaryheap() {
        let mut binaryheap = BinaryHeap::<usize>::new();
        binaryheap.add(4);
        binaryheap.add(9);
        binaryheap.add(8);
        binaryheap.add(17);
        binaryheap.add(26);
        binaryheap.add(50);
        binaryheap.add(16);
        binaryheap.add(19);
        binaryheap.add(69);
        binaryheap.add(32);
        binaryheap.add(93);
        binaryheap.add(55);
        binaryheap.add(6);
        assert_eq!(&Some(4), binaryheap.a.get(0).unwrap());
        assert_eq!(&Some(9), binaryheap.a.get(1).unwrap());
        assert_eq!(&Some(6), binaryheap.a.get(2).unwrap());
        assert_eq!(&Some(17), binaryheap.a.get(3).unwrap());
        assert_eq!(&Some(26), binaryheap.a.get(4).unwrap());
        assert_eq!(&Some(8), binaryheap.a.get(5).unwrap());
        assert_eq!(&Some(16), binaryheap.a.get(6).unwrap());
        assert_eq!(&Some(19), binaryheap.a.get(7).unwrap());
        assert_eq!(&Some(69), binaryheap.a.get(8).unwrap());
        assert_eq!(&Some(32), binaryheap.a.get(9).unwrap());
        assert_eq!(&Some(93), binaryheap.a.get(10).unwrap());
        assert_eq!(&Some(55), binaryheap.a.get(11).unwrap());
        assert_eq!(&Some(50), binaryheap.a.get(12).unwrap());
        assert_eq!(Some(4), binaryheap.remove());
        assert_eq!(&Some(6), binaryheap.a.get(0).unwrap());
        assert_eq!(&Some(9), binaryheap.a.get(1).unwrap());
        assert_eq!(&Some(8), binaryheap.a.get(2).unwrap());
        assert_eq!(&Some(17), binaryheap.a.get(3).unwrap());
        assert_eq!(&Some(26), binaryheap.a.get(4).unwrap());
        assert_eq!(&Some(50), binaryheap.a.get(5).unwrap());
        assert_eq!(&Some(16), binaryheap.a.get(6).unwrap());
        assert_eq!(&Some(19), binaryheap.a.get(7).unwrap());
        assert_eq!(&Some(69), binaryheap.a.get(8).unwrap());
        assert_eq!(&Some(32), binaryheap.a.get(9).unwrap());
        assert_eq!(&Some(93), binaryheap.a.get(10).unwrap());
        assert_eq!(&Some(55), binaryheap.a.get(11).unwrap());
        //println!("{:?}", binaryheap);
    }
}
