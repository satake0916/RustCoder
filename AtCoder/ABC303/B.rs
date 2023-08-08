use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize;n]; m]
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in (i+1)..=n {
            let mut is = true;
            for v in a.iter() {
                for w in v.windows(2) {
                    if (i == w[0] && j == w[1]) || (i == w[1] && j == w[0]) {
                        is = false;
                    }
                }
            }
            if is {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
