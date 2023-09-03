use itertools::Itertools;
use proconio::*;
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut roulettes = vec![];
    for _ in 0..n {
        input! {
            c: f64,
            p: usize,
            s: [usize; p]
        }
        roulettes.push((c, s));
    }

    // 前処理
    for (c, s) in roulettes.iter_mut() {
        let zeros = s.iter().filter(|x| **x == 0_usize).count() as f64;
        let p = s.len() as f64;
        *c = p / (p - zeros) * *c;
        *s = s.iter().filter(|x| **x != 0).map(|x| *x).collect_vec();
    }

    // dp[i]: iポイント持っているときに、mポイントに到達するのに必要な金額
    let mut dp = vec![std::f64::MAX; m + 1];
    dp[m] = 0_f64;

    for i in (0..m).rev() {
        for (c, s) in roulettes.iter() {
            let cost = c + s
                .iter()
                .map(|x| if i + x > m { 0_f64 } else { dp[i + x] })
                .sum::<f64>()
                / s.len() as f64;
            dp[i] = dp[i].min(cost);
        }
    }

    println!("{:.7}", dp[0]);
}
