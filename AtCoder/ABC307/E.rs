use itertools::Itertools;
use proconio::*;
use proconio::{marker::*, source::line::LineSource};
use std::io::BufReader;
use std::{cmp, iter};
use std::{cmp::*, collections::*};
use superslice::Ext;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut dp = vec![vec![0_usize; 2]; n];
    // dp[i][j]: i人めまで決めて、j = 0 最後の人が0人めの人と同じ数を持っている, j = 1 違う数

    dp[0][0] = 1;
    for i in 0..(n - 1) {
        dp[i + 1][1] += dp[i][0] * (m - 1);
        dp[i + 1][0] += dp[i][1];
        dp[i + 1][1] += dp[i][1] * (m - 2);

        dp[i + 1][0] %= MOD;
        dp[i + 1][1] %= MOD;
    }
    println!("{}", dp[n - 1][1] * m % MOD)
}
