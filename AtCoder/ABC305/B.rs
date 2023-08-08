use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        p: char,
        q: char
    }

    let v: [i32; 7] = [0, 3, 4, 8, 9, 14, 23];
    println!("{}", (v[p as usize - 'A' as usize] - v[q as usize - 'A' as usize]).abs())
}
