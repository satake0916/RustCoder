use itertools::Itertools;
use proconio::*;
use proconio::{marker::*, source::line::LineSource};
use std::io::BufReader;
use std::{cmp, iter};
use std::{cmp::*, collections::*};
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: [(usize, usize); n]
    }

    let mut hole = vec![vec![false; w+1]; h+1];
    for (a, b) in t { hole[a][b] = true }

    let mut dp = vec![vec![0usize; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            if hole[i][j] {
                dp[i][j] = 0;
            } else {
                dp[i][j] = cmp::min(dp[i - 1][j - 1], cmp::min(dp[i-1][j], dp[i][j-1])) + 1;
            }
        }
    }

    // println!("{:?}", dp);
    println!("{}", dp.iter().flatten().sum::<usize>());
}
