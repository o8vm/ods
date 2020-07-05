#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter10::binaryheap::BinaryHeap;

pub fn heapsort<T: PartialOrd + Clone>(a: &mut [T]) {
    BinaryHeap::sort(a);
}

#[cfg(test)]
mod test {
    use super::heapsort;
    use rand::distributions::Standard;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_heapsort() {
        let mut a = [13, 8, 5, 2, 4, 0, 6, 9, 7, 3, 12, 1, 10, 11];
        heapsort(&mut a);
        assert_eq!(&a, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
        //println!("{:?}", a);
        let mut a = [0];
        heapsort(&mut a);
        assert_eq!(&a, &[0]);

        let mut rng = thread_rng();
        for _ in 0u32..50000u32 {
            let len: usize = rng.gen();
            let mut v: Vec<isize> = rng.sample_iter(&Standard).take((len % 32) + 1).collect();
            heapsort(&mut v);
            for i in 0..v.len() - 1 {
                assert!(v[i] <= v[i + 1])
            }
        }
    }
}
