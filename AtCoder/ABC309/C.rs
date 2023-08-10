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
        k: usize,
        d: [(usize, usize); n]
    }

    let mut l = 1000_000_001_usize;
    let mut r = 0;
    while l - r > 1 {
        let mid = (l + r) / 2;

        let req: usize = d.iter().filter(|(a, _)| a >= &mid).map(|(_, b)| b).sum();
        if req <= k {
            l = mid;
        }else{
            r = mid;
        }
    }

    println!("{}", l);
}
