use itertools::Itertools;
use proconio::{
    marker::{Chars, Usize1},
    *,
};
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        query: [(usize, usize, char); q]
    }

    let mut changed = s;

    for (t, x, c) in query.iter() {
        if *t == 1 {
            changed[*x - 1] = *c;
        }
    }

    let mut ans = vec!['0'; n];
    for (t, x, c) in query.iter().rev() {
        dbg!(*t);
        if *t == 1 {
            ans[*x - 1] = *c;
        } else {
            println!(
                "{}",
                (0..n)
                    .map(|i| if ans[i] != '0' {
                        ans[i].to_string()
                    } else {
                        if *t == 2 {
                            changed[i].to_lowercase().to_string()
                        } else {
                            changed[i].to_uppercase().to_string()
                        }
                    })
                    .join("")
            );
            return;
        }
    }

    println!("{}", changed.iter().join(""));
}
