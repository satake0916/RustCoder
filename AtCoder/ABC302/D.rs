use proconio::*;
use superslice::Ext;
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        a: [usize; n],
        mut b: [usize; m]
    }

    b.sort();
    let mut ans = 0;

    for a in a {
        let r = a + d;
        let l = if a < d { 0 } else {a - d};
        if b.upper_bound(&r) - b.lower_bound(&l) > 0 {
            ans = ans.max(a + b[b.upper_bound(&r) - 1]);
        }
    }

    println!("{}", if ans > 0 {ans.to_string()} else {"-1".to_string()});
}