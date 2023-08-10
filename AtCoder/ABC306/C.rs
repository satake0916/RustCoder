use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MAX: usize = 31;

fn main() {
    input! {
        n: usize,
        a: [usize; 3 * n],
    }
    let mut mp = vec![0; n+1];
    let mut ans = vec![];
    for a in a {
        if mp[a] == 1 {
            ans.push(a);
        }
        mp[a] += 1;
    }

    println!("{}", ans.iter().join(" "));
}
