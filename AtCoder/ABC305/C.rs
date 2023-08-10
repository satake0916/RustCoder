use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut hv = vec![false; h];
    let mut wv = vec![false; w];

    for i in 0..h {
        for j in 0..w {
            hv[i] |= s[i][j] == '#';
            wv[j] |= s[i][j] == '#';
        }
    }

    for i in 0..h {
        for j in 0..w {
            if (s[i][j] != '#') && hv[i] && wv[j] {
                println!("{} {}", i+1, j+1);
                return
            }
        }
    }

}
