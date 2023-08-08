use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    for i in (1..n).rev() {
        let x = a[i-1];
        let y = a[i];
        if x < y {
            a.splice(i..i, (x+1)..y);
        }else{
            a.splice(i..i, ((y+1)..x).rev());
        }
    }

    println!("{}", a.iter().join(" "));
}
