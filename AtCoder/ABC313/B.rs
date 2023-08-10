use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;
use crate::marker::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [(usize, usize); m]
    }

    let mut v = vec![1; n];
    for (a, b) in p.iter() {
        v[b-1] = 0;
    }
    if v.iter().sum::<usize>() != 1 {
        println!("-1");
    }else {
        println!("{}", v.iter().position(|x| x == &1).unwrap() + 1)
    }
}
