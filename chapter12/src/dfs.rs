#![allow(clippy::many_single_char_names,clippy::explicit_counter_loop, clippy::redundant_closure)]
use super::adjacencylists::AdjacencyLists;
use chapter01::interface::{Graph, Stack};
use chapter03::sllist::SLList;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Color {
    White,
    Grey,
    Black,
}

fn do_dfs(g: &AdjacencyLists, i: usize, c: &mut [Color]) {
    if let Some(e) = c.get_mut(i) {
        *e = Color::Grey
    }
    let edges = g.out_edges(i);
    for j in edges.into_iter() {
        if let Some(Color::White) = c.get(j) {
            if let Some(e) = c.get_mut(j) {
                *e = Color::Grey
            }
            do_dfs(g, j, c);
        }
    }
    if let Some(e) = c.get_mut(i) {
        *e = Color::Black
    }
}

pub fn dfs(g: &AdjacencyLists, r: usize) {
    let mut c = vec![Color::White; g.nvertices()];
    do_dfs(g, r, &mut c);
}

pub fn dfs2(g: &AdjacencyLists, r: usize) {
    let mut c = vec![Color::White; g.nvertices()];
    let mut s = SLList::new();
    s.push(r);
    while let Some(i) = s.pop() {
        if let Some(Color::White) = c.get(i) {
            if let Some(e) = c.get_mut(i) {
                *e = Color::Grey
            }
            let edges = g.out_edges(i);
            for j in edges.into_iter() {
                s.push(j);
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
    fn test_dfs() {
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
        dfs(&adjm, 0);
        println!("done");
        dfs2(&adjm, 0);
        println!("done2");
    }
}
