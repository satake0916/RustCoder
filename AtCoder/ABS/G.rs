use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ma = a;
    ma.sort_by(|a, b| b.cmp(a));
    let mut sa = 0;
    let mut sb = 0;
    for (i, x) in ma.iter().enumerate() {
        if i % 2 == 0 {
            sa += x;
        }else{
            sb += x;
        }
    }

    println!("{}", sa - sb);
}
