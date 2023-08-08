use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }

    println!("{}", d.iter().unique().count());
}
