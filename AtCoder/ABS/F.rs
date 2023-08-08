use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    let ans: usize = (0..=n).filter(|x| {
        let mut sum = 0;
        let mut y = *x;
        while y > 0 {
            sum += y % 10;
            y /= 10;
        }
        a <= sum && sum <= b
    }).sum();

    println!("{}", ans);
}
