use std::collections::VecDeque;

use proconio::{*, marker::Usize1};
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        n: usize,
        m: usize,
        parent: [Usize1; n-1],
        mut insurance: [(Usize1, usize); m]
    }

    // 前処理
    let mut children = vec![vec![]; n];
    for (i, p) in parent.iter().enumerate() {
        children[*p].push(i + 1);
    }

    let mut des = vec![0; n];
    for (x, y) in insurance {
        des[x] = des[x].max(y + 1);
    }

    // 始祖から順番に保険に加入しているかみていく
    let mut ans = 0;
    for i in 0..n {
        if des[i] > 0 {
            ans += 1;
            for child in &children[i]{
                des[*child] = des[*child].max(des[i] - 1);
            }
        }
    }

    println!("{}", ans);
}