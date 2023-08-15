
use itertools::Itertools;
use proconio::{marker::*, source::line::LineSource};
use proconio::*;
use superslice::Ext;
use std::io::BufReader;
use std::iter;
use std::{
    cmp::{*},
    collections::*,
};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n_1: usize,
        n_2: usize,
        m: usize,
        e: [(usize, usize); m]
    }
    let n = n_1 + n_2;
    let mut edges = vec![vec![]; n+1];

    for e in e {
        edges[e.0].push(e.1);
        edges[e.1].push(e.0);
    }

    let mut dis = vec![std::usize::MAX; n+1];
    let mut que = Deque::new();
    dis[1] = 0;
    que.push_back(1);

    let mut w_1 = 0;
    while let Some(top) = que.pop_front() {
        let w = dis[top];
        for nx in edges[top].iter() {
            if dis[*nx] > w + 1 {
                dis[*nx] = w + 1;
                w_1 = w_1.max(w+1);
                que.push_back(*nx);
            }
        }
    }

    let mut w_2 = 0;
    que.clear();
    dis[n] = 0;
    que.push_back(n);
    while let Some(top) = que.pop_front() {
        let w = dis[top];
        for nx in edges[top].iter() {
            if dis[*nx] > w + 1 {
                dis[*nx] = w + 1;
                w_2 = w_2.max(w + 1);
                que.push_back(*nx);
            }
        }
    }

    println!("{}", w_1 + w_2 + 1);
}