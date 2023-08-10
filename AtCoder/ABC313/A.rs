use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;
use crate::marker::*;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mx = p.iter().skip(1).max().unwrap_or(&0);
    println!("{}", if mx < &p[0] {0} else {mx - p[0] + 1});
}
