use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*, arch::x86_64::_CMP_UNORD_Q};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    }

    let mut st = Set::new();
    for temp in s.iter() {
        let mut t = temp.clone();
        t.reverse();
        if !(st.contains(&temp) || st.contains(&t)) {
            st.insert(temp);
        }
    }

    println!("{}", st.len());
}
