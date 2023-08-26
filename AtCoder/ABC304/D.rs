use std::collections::BTreeMap;

use proconio::*;
use superslice::Ext;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        straw: [(usize, usize); n],
        a: usize,
        mut a_arr: [usize; a],
        b: usize,
        mut b_arr: [usize; b]
    }

    a_arr.push(w);
    b_arr.push(h);

    let mut mp = BTreeMap::new();

    for (p, q) in straw {
        let a_idx = a_arr.lower_bound(&p);
        let b_idx = b_arr.lower_bound(&q);
        *mp.entry((a_idx, b_idx)).or_insert(0) += 1;
    }

    let mut mx = 0;
    let mut mn = n;

    for (_k, v) in mp.iter() {
        mx = mx.max(*v);
        mn = mn.min(*v);
    }
    if mp.len() < (a + 1) * (b + 1) {
        mn = 0;
    }

    println!("{} {}", mn, mx)
}
