use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*, arch::x86_64::_CMP_UNORD_Q};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        edge: [usize; n]
    }

    let mut v = vec![false; n+1];
    let mut ans = 0_usize;

    let mut cur = 1;
    loop {
        if v[cur] {
            ans = cur;
            break;
        } else {
            v[cur] = true;
            cur = edge[cur-1];
        }
    }

    let mut al = vec![cur];
    cur = edge[cur-1];
    while cur != ans {
        al.push(cur);
        cur = edge[cur-1];
    }

    println!("{}\n{}", al.len(), al.iter().join(" "))
}
