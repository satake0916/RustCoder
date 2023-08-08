use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
    }

    let mut list = vec![(0, 0, 0)];
    for _ in 0..n {
        input! {
            t: i32,
            x: i32,
            y: i32,
        }
        list.push((t, x, y))
    }

    println!(
        "{}",
        if (0..n).all(|i| {
            let dt = list[i + 1].0 - list[i].0;
            let dis = (list[i + 1].1 - list[i].1).abs() + (list[i + 1].2 - list[i].2).abs();
            dt >= dis && dt % 2 == dis % 2
        }) {
            "Yes"
        } else {
            "No"
        }
    );
}
