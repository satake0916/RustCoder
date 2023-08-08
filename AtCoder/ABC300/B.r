use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;
use crate::marker::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Bytes; h],
        b: [Bytes; h]
    }

    let mut ans = false;
    for s in 0..h {
        for t in 0..w {
            if a == b {
                ans = true;
            }
            for x in a.iter_mut() {
                x.rotate_left(1);
            }
        }
        a.rotate_left(1);
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
