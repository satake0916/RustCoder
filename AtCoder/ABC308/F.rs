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
        p: [usize; n],
        l: [usize; m],
        d: [usize; m]
    }

    let mut ans = p.iter().sum::<usize>();
    let mut ms = BTreeSet::new();
    for (i, x) in p.iter().enumerate() {
        ms.insert((x, i));
    }

    let mut coupon = d.iter().zip(l.iter()).collect_vec();
    coupon.sort_by(|a, b| b.0.cmp(a.0));

    // println!("{:?}", ms);
    // println!("{:?}", coupon);

    for (d, l) in coupon.iter() {
        if let Some((x, i)) = ms.range((*l, 0)..).next() {
            ans -= *d;
            ms.remove(&(*x, *i));
        }
    }

    println!("{}", ans);
}
