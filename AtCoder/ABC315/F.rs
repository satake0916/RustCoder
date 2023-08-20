use std::cmp;

use proconio::*;

fn main() {
    input! {
      n: usize,
      point: [(i32, i32); n]
    }

    // dp[i][j]: j回ペナルティを受けて、i地点にいる最小スコア
    let mut dp = vec![vec![10f64.powi(10); 30]; n];
    dp[0][0] = 0_f64;

    for i in 0..(n - 1) {
        for j in 0..n {
            // dp[i][j]について配る

            // 地点iからk個飛ばして、i+k+1地点に行く
            for k in (0..(n - 1 - i)).take(30) {
                if j + k > cmp::min(29, n - 2) {
                    break;
                }
                let now = point[i];
                let next = point[i + k + 1];
                let dis = (((now.0 - next.0).pow(2) + (now.1 - next.1).pow(2)) as f64).sqrt();

                dp[i + k + 1][j + k] = dp[i + k + 1][j + k].min(dp[i][j] + dis);
            }
        }
    }

    let mut ans = dp[n - 1][0];
    for (i, dp) in dp[n - 1].iter().skip(1).enumerate() {
        ans = ans.min(*dp + (1i64 << i) as f64);
    }

    println!("{:.7}", ans);
}
