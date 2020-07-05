#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter01::interface::Graph;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdjacencyMatrix {
    n: usize,
    a: Vec<Vec<bool>>,
}

impl AdjacencyMatrix {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            a: vec![vec![false; n]; n],
        }
    }
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut bool> {
        if let Some(ii) = self.a.get_mut(i) {
            ii.get_mut(j)
        } else {
            None
        }
    }
    fn get(&self, i: usize, j: usize) -> Option<&bool> {
        if let Some(ii) = self.a.get(i) {
            ii.get(j)
        } else {
            None
        }
    }
}

impl Graph for AdjacencyMatrix {
    fn add_edge(&mut self, i: usize, j: usize) {
        if let Some(e) = self.get_mut(i, j) {
            *e = true;
        }
    }
    fn remove_edge(&mut self, i: usize, j: usize) {
        if let Some(e) = self.get_mut(i, j) {
            *e = false;
        }
    }
    fn has_edge(&self, i: usize, j: usize) -> bool {
        if let Some(e) = self.get(i, j) {
            *e
        } else {
            false
        }
    }
    fn out_edges(&self, i: usize) -> Vec<usize> {
        let mut edges = vec![];
        for j in 0..self.n {
            if let Some(true) = self.get(i, j) {
                edges.push(j);
            }
        }
        edges
    }
    fn in_edges(&self, i: usize) -> Vec<usize> {
        let mut edges = vec![];
        for j in 0..self.n {
            if let Some(true) = self.get(j, i) {
                edges.push(j);
            }
        }
        edges
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;
    #[test]
    fn test_adjacencymatrix() {
        let n = 50;
        let mut adjm = AdjacencyMatrix::new(n);
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut rng = thread_rng();
        // add test
        for _ in 0..(5 * n) {
            let (i, j) = (rng.gen_range(0, n), rng.gen_range(0, n));
            if !set.contains(&(i, j)) {
                set.insert((i, j));
                adjm.add_edge(i, j);
            }
        }
        for i in 0..n {
            for j in 0..n {
                assert_eq!(adjm.has_edge(i, j), set.contains(&(i, j)));
            }
        }
        // remove test
        for _ in 0..n {
            let (i, j) = (rng.gen_range(0, n), rng.gen_range(0, n));
            if set.contains(&(i, j)) {
                set.remove(&(i, j));
                adjm.remove_edge(i, j);
            }
        }
        for i in 0..n {
            for j in 0..n {
                assert_eq!(adjm.has_edge(i, j), set.contains(&(i, j)));
            }
        }
        // check that in and out degrees are correctly computed
        for i in 0..n {
            let mut oe = 0;
            let mut ie = 0;
            for _ in set.iter().filter(|e| e.0 == i) {
                oe += 1;
            }
            for _ in set.iter().filter(|e| e.1 == i) {
                ie += 1;
            }
            assert_eq!(adjm.out_edges(i).len(), oe);
            assert_eq!(adjm.in_edges(i).len(), ie);
        }
    }
}
