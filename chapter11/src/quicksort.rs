#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
pub fn quicksort<T: PartialOrd + Clone>(a: &mut [T]) {
    do_sort(a, 0, a.len());
}
fn do_sort<T: PartialOrd + Clone>(a: &mut [T], i: usize, n: usize) {
    if n <= 1 {
        return;
    }
    let x = a[i + rand::random::<usize>() % n].clone();
    let mut p = i;
    let mut j = i;
    let mut q = i + n;
    while j < q {
        if a.get(j) < Some(&x) {
            a.swap(j, p);
            j += 1;
            p += 1;
        } else if a.get(j) > Some(&x) {
            q -= 1;
            a.swap(j, q);
        } else {
            j += 1;
        }
    }
    do_sort(a, i, p - i);
    do_sort(a, q, n - (q - i));
}

#[cfg(test)]
mod test {
    use super::quicksort;
    use rand::distributions::Standard;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_quicksort() {
        let mut a = [13, 8, 5, 2, 4, 0, 6, 9, 7, 3, 12, 1, 10, 11];
        quicksort(&mut a);
        assert_eq!(&a, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
        let mut a = [0, 0];
        quicksort(&mut a);
        assert_eq!(&a, &[0, 0]);
        let mut a = [2, 0];
        quicksort(&mut a);
        assert_eq!(&a, &[0, 2]);

        let mut rng = thread_rng();
        for _ in 0u32..50000u32 {
            let len: usize = rng.gen();
            let mut v: Vec<isize> = rng.sample_iter(&Standard).take((len % 32) + 1).collect();
            quicksort(&mut v);
            for i in 0..v.len() - 1 {
                assert!(v[i] <= v[i + 1])
            }
        }
    }
}
