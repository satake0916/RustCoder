use num_integer::gcd;
use proconio::*;
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        mut n: usize,
        m: usize,
        mut op: [(usize, usize); m]
    }

    op.sort_by_key(|k| k.1);

    let mut cost = 0;

    for (a, c) in op {
        let g = gcd(n, a);
        cost += (n / g - 1) * g * c;
        n = g;

        if n <= 1 {
            break;
        }
    }

    if n == 1 {
        println!("{}", cost);
    } else {
        println!("-1")
    }
}