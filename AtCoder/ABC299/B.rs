use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{
    cmp::{self, *},
    collections::*,
};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n]
    }

    let color = if c.iter().any(|c| *c == t) { t } else { c[0] };
    let mx = (0..n).filter(|i| c[*i] == color).max_by_key(|i| r[*i]).unwrap();
    println!("{}", mx + 1);
}
