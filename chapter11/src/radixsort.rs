#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
pub fn radixsort(a: &mut [usize]) {
    let d = 8;
    let w = 8 * std::mem::size_of::<usize>();
    for p in 0..(w / d) {
        let mut c = vec![0usize; 1 << d];
        let mut b = vec![0usize; a.len()];
        for ai in a.iter() {
            c[(*ai >> (d * p)) & ((1 << d) - 1)] += 1;
        }
        for i in 1..(1 << d) {
            c[i] += c[i - 1];
        }
        for ai in a.iter().rev() {
            c[(*ai >> (d * p)) & ((1 << d) - 1)] -= 1;
            b[c[(*ai >> (d * p)) & ((1 << d) - 1)]] = *ai;
        }
        a.copy_from_slice(&b);
    }
}

#[cfg(test)]
mod test {
    use super::radixsort;
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_radixsort() {
        let mut a = [7, 2, 9, 0, 1, 2, 0, 9, 7, 4, 4, 6, 9, 1, 0, 9, 3, 2, 5, 9];
        radixsort(&mut a);
        assert_eq!(
            &a,
            &[0, 0, 0, 1, 1, 2, 2, 2, 3, 4, 4, 5, 6, 7, 7, 9, 9, 9, 9, 9]
        );
        let mut rng = thread_rng();
        for _ in 0u32..50000u32 {
            let len: usize = rng.gen();
            let die_range = Uniform::new_inclusive(1, 1000);
            let mut v: Vec<usize> = rng.sample_iter(die_range).take((len % 32) + 1).collect();
            radixsort(&mut v);
            for i in 0..v.len() - 1 {
                assert!(v[i] <= v[i + 1])
            }
        }
    }
}
