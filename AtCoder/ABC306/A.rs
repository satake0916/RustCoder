use proconio::*;
use std::collections::*;
use std::cmp::*;

fn main() {
    input!{
        n: usize,
        s: String
    }
    for i in s.chars() {
        print!{"{}{}", i, i}
    }
}
