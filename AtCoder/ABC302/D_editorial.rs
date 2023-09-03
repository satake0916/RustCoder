use proconio::*;
use superslice::Ext;
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        mut a: [i64; n],
        mut b: [i64; m]
    }

    a.sort();
    b.sort();

    while !a.is_empty() && !b.is_empty() {
        let n = a.len();
        let m = b.len();
        if (a[n - 1] - b[m - 1]).abs() <= d {
            println!("{}", a[n - 1] + b[m - 1]);
            return;
        }
        if a[n - 1] > b[m - 1] {
            a.pop();
        } else {
            b.pop();
        }
    }

    println!("-1");
}
