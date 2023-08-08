use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        s: String
    }

    let strs = ["dream", "dreamer", "erase", "eraser"];
    let mut end = s.len();
    while end > 0 {
        let mut ok = false;
        for x in strs.iter() {
            if s[..end].ends_with(x) {
                end -= x.len();
                ok = true;
            }
        }
        if !ok {
            println!("NO");
            return
        }
    }
    println!("YES");
}
