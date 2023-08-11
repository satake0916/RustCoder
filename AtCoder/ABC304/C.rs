use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        d: i32,
        p: [(i32, i32); n]
    }
    let mut edge = vec![vec![]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            if (p[i].0 - p[j].0) * (p[i].0 - p[j].0) + (p[i].1 - p[j].1) * (p[i].1 - p[j].1)
                <= d * d
            {
                edge[i].push(j);
                edge[j].push(i);
            }
        }
    }

    let mut is_dead = vec![false; n];
    is_dead[0] = true;

    let mut q = Deque::new();
    q.push_front(0);

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        edge[cur].iter().for_each(|x| {
            if !is_dead[*x] {
                is_dead[*x] = true;
                q.push_back(*x);
            }
        });
    }

    println!(
        "{}",
        is_dead
            .iter()
            .map(|f| if *f { "Yes" } else { "No" })
            .join("\n")
    );
}
