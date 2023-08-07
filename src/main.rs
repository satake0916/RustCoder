use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::cmp;
use std::collections::*;


fn main() {
    input! {
        s: [usize; 64]
    }

    let ans: usize = (0..64).map(|x| s[x] << x).sum();

    println!("{}", ans);
}
