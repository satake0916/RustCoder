use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        s: String
    }
    println!("{}", s.chars().map(|c| c as usize - 48).sum::<usize>());
}
