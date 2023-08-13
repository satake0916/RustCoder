
use proconio::marker::*;
use proconio::*;
use std::{
    cmp::{*},
    collections::*,
};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    let n = read();

    let mut r = 1; // 必ず0
    let mut l = n; // 必ず1

    while l - r > 1 {
        let mid = (r + l) / 2;
        println!("? {}", mid);
        let res = read();
        if res == 1 {
            l = mid;
        }else{
            r = mid;
        }
    }

    println!("! {}", r);
}

fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}