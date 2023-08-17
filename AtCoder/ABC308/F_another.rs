use itertools::Itertools;
use proconio::*;
use proconio::{marker::*, source::line::LineSource};
use std::io::BufReader;
use std::{cmp, iter};
use std::{cmp::*, collections::*};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
        l: [usize; m],
        d: [usize; m]
    }

    let mut ans = 0;
    let mut index = 0;
    let mut coupon = l.into_iter().zip(d.into_iter()).collect_vec();
    let mut canuse_coupon = BinaryHeap::new();

    coupon.sort();
    p.sort();

    coupon.push((std::usize::MAX, 0));

    for p in p {
        ans += p;
        while coupon[index].0 <= p {
            canuse_coupon.push(coupon[index].1);
            index += 1;
        }

        if let Some(discount) = canuse_coupon.pop() {
            ans -= discount;
        }
    }

    println!("{}", ans);
}
