use proconio::{*, marker::Chars};
#[allow(unused_macros)]
macro_rules ! dbg {($ ($ x : tt ) * ) => {{# [cfg (debug_assertions ) ] {std :: dbg ! ($ ($ x ) * ) } # [cfg (not (debug_assertions ) ) ] {($ ($ x ) * ) } } } }

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars
    }

    let mut dp = vec![vec![0; 8]; 3];

    for i in 0..n {
        let mut next = dp.clone();
        match s[i] {
            'M' => {
                next[0][1 << a[i]] += 1;
            },
            'E' => {
                for j in 0..8 {
                    next[1][j | (1 << a[i])] += dp[0][j];
                }
            },
            'X' => {
                for j in 0..8 {
                    next[2][j | (1 << a[i])] += dp[1][j];
                }
            },
            _ => {}
        }
        dbg!(&next);
        dp = next;
    }

    let mex = |x| -> usize {
        for i in 0..3 {
            if x & (1 << i) == 0 {
                return i;
            }
        }
        return 3;
    };

    dbg!(mex(1));
    dbg!(mex(2));
    dbg!(mex(3));
    dbg!(mex(7));

    println!("{}", dp[2].iter().enumerate().map(|(i, x)| mex(i) * x).sum::<usize>());
}