use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::{*, self}, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: usize = 998244353;

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();

    let mut dp = vec![vec![0; n+1]; n+1];

    // dp[i][j] i個までみたとき、j個かっこが開いている場合の数
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 0..=n {
            if s[i-1] != ')' && j > 0 {
                dp[i][j] += dp[i-1][j-1]
            }
            if s[i-1] != '(' && j < n {
                dp[i][j] += dp[i-1][j+1]
            }
            dp[i][j] %= MOD;
        }
    }

    println!("{}", dp[n][0]);
}
