#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use chapter01::interface::{Graph, List};
use chapter02::arraystack::Array as ArrayStack;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdjacencyLists {
    n: usize,
    adj: Vec<ArrayStack<usize>>,
}

impl AdjacencyLists {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![ArrayStack::new(); n],
        }
    }
    pub fn nvertices(&self) -> usize {
        self.n
    }
}

impl Graph for AdjacencyLists {
    fn add_edge(&mut self, i: usize, j: usize) {
        if let Some(e) = self.adj.get_mut(i) {
            e.add(e.size(), j);
        }
    }
    fn remove_edge(&mut self, i: usize, j: usize) {
        if let Some(e) = self.adj.get_mut(i) {
            for k in 0..e.size() {
                if e.get(k) == Some(j) {
                    e.remove(k);
                    return;
                }
            }
        }
    }
    fn has_edge(&self, i: usize, j: usize) -> bool {
        match self.adj.get(i) {
            Some(e) => e.contains(j),
            None => false,
        }
    }
    fn out_edges(&self, i: usize) -> Vec<usize> {
        let mut edges = vec![];
        if let Some(e) = self.adj.get(i) {
            for k in 0..e.size() {
                if let Some(u) = e.get(k) {
                    edges.push(u)
                }
            }
        }
        edges
    }
    fn in_edges(&self, i: usize) -> Vec<usize> {
        let mut edges = vec![];
        for j in 0..self.n {
            if let Some(e) = self.adj.get(j) {
                if e.contains(i) {
                    edges.push(j);
                }
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
    fn test_adjacencylists() {
        let n = 50;
        let mut adjm = AdjacencyLists::new(n);
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
