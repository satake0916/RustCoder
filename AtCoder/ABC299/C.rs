
use proconio::marker::*;
use proconio::*;
use std::{
    cmp::{*},
    collections::*,
};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    if s.iter().all(|c| *c == 'o') {
        println!("-1");
        return
    }

    let mut count = 0;
    let mut mx = -1;
    for c in s {
        if c == 'o' {
            count += 1;
            mx = mx.max(count);
        } else if c == '-' {
            count = 0;
        }
    }
    println!("{}", mx);
}
