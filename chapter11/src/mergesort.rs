pub fn merge_sort<T: PartialOrd + Clone>(a: &mut [T]) {
    let mut w = a.to_vec();
    if a.len() > 1 {
        let mid = a.len() / 2;
        let (a0, a1) = a.split_at_mut(mid);
        merge_sort(a0);
        merge_sort(a1);
        merge(a0, a1, &mut w);
    }
    a.clone_from_slice(&w);
}

fn merge<T: PartialOrd + Clone>(a0: &mut [T], a1: &mut [T], a: &mut [T]) {
    let mut i0 = 0; let mut i1 = 0;
    for ai in a.iter_mut() {
        if i0 == a0.len() {
            *ai = a1[i1].clone();
            i1 += 1;
        } else if i1 == a1.len() {
            *ai = a0[i0].clone();
            i0 += 1;
        } else if a0[i0] < a1[i1] {
            *ai = a0[i0].clone();
            i0 += 1;
        } else {
            *ai = a1[i1].clone();
            i1 += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::merge_sort;
    #[test]
    fn test_mergesort() {
        let mut a = [13, 8, 5, 2, 4, 0, 6, 9, 7, 3, 12, 1, 10, 11];
        merge_sort(&mut a);
        assert_eq!(&a, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
        //println!("{:?}", a);
    }
}