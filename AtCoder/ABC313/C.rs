use proconio::*;
use proconio::marker::*;
use std::{collections::*, cmp};
use itertools::*;
 
type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();
    let quo = sum / n;

    let mut up = 0;
    let mut down = 0;
    

    for x in a {
        if x <= quo {
            down += quo - x;
        }else{
            up += x - (quo + 1);
        }
    }

    println!("{}", cmp::max(up, down));

}
