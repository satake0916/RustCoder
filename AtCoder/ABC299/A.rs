use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{
    cmp::{self, *},
    collections::*,
};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut count = 0;
    for c in s {
        match c {
            '|' => count += 1,
            '*' => {
                if count == 1 {
                    println!("in");
                }else{
                    println!("out");
                }
                break;
            }
            _ => ()
        }
    }
}
