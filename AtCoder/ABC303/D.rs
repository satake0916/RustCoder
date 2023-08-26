use std::cmp;

use proconio::{*, marker::Chars};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars
    }

    let n = s.len();
    let mut dp = vec![vec![std::usize::MAX; 2]; n+1];

    dp[0][0] = 0;
    dp[0][1] = z;

    for i in 0..n {
        let (s, t) = if s[i] == 'a' { (0, 1) } else { (1, 0) };
        dp[i+1][s] = dp[i+1][s].min(dp[i][s] + x);
        dp[i+1][t] = dp[i+1][t].min(dp[i][t] + y);

        dp[i+1][s] = dp[i+1][s].min(dp[i][t] + z + x);
        dp[i+1][t] = dp[i+1][t].min(dp[i][s] + z + y);
        
    }

    println!("{}", cmp::min(dp[n][0], dp[n][1]));
}
