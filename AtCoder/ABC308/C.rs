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
        t: [(usize, usize); n]
    }

    let mut v = (1..=n).zip(t.iter()).collect_vec();

    v.sort_by(|x, y| {
        let (ax, bx) = x.1;
        let (ay, by) = y.1;
        (ay * bx).cmp(&(ax * by))
    });

    println!("{}", v.iter().map(|(i, _)| i).join(" "));
}
