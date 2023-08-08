use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    println!(
        "{}",
        a.into_iter()
            .map(|x| {
                let mut y = x;
                let mut count = 0;
                while y % 2 == 0 {
                    y /= 2;
                    count += 1;
                }
                count
            })
            .min()
            .unwrap()
    );
}
