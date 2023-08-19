use ascii::AsAsciiStr;
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
        s: Chars
    }
    let mut count = 0;
    let mut deq = VecDeque::new();

    for c in s {
        match c {
            '(' => {
                deq.push_back(c);
                count += 1;
            },
            ')' => {
                if count >  0 {
                    let mut pre = deq.pop_back().unwrap();
                    while pre != '(' {
                        pre = deq.pop_back().unwrap();
                    }
                    count -= 1;
                } else {
                    deq.push_back(c);
                }
            },
            _ => deq.push_back(c)
        }
    }

    println!("{}", deq.iter().join(""));
}
