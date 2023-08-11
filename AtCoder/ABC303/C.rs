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
        m: usize,
        mut h: i32,
        k: i32,
        s: Chars,
        p: [(i32, i32); m]
    }

    let mut st = p.into_iter().collect::<Set<_>>();

    let mut cur = (0, 0);
    for c in s {
        h -= 1;
        match c {
            'R' => cur.0 += 1,
            'L' => cur.0 -= 1,
            'U' => cur.1 += 1,
            'D' => cur.1 -= 1,
            _ => panic!()
        }

        if h < 0 {
            println!("No");
            return;
        } else {
            if h < k && st.contains(&cur) {
                h = k;
                st.remove(&cur);
            }
        }
    }

    println!("Yes");
}
