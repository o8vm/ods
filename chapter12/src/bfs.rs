#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use super::adjacencylists::AdjacencyLists;
use chapter01::interface::{Graph, Queue};
use chapter03::sllist::SLList;

pub fn bfs(g: &AdjacencyLists, r: usize) {
    let mut seen = vec![false; g.nvertices()];
    let mut q = SLList::new();
    q.add(r);
    if let Some(e) = seen.get_mut(r) {
        *e = true
    }
    while let Some(i) = q.remove() {
        let edges = g.out_edges(i);
        for j in edges.into_iter() {
            if !seen[j] {
                q.add(j);
                if let Some(e) = seen.get_mut(j) {
                    *e = true
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;
    #[test]
    fn test_bfs() {
        let n = 50;
        let mut adjm = AdjacencyLists::new(n);
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut rng = thread_rng();
        for _ in 0..(5 * n) {
            let (i, j) = (rng.gen_range(0, n), rng.gen_range(0, n));
            if !set.contains(&(i, j)) {
                set.insert((i, j));
                adjm.add_edge(i, j);
            }
        }
        bfs(&adjm, 0);
        println!("done");
    }
}
