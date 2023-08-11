use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

pub struct PermutationIterator<T: Ord + Clone> {
    li: Vec<T>,
    is_finished: bool,
}
impl<T: Ord + Clone> PermutationIterator<T> {
    pub fn new(mut li: Vec<T>) -> PermutationIterator<T> {
        let is_finished = li.is_empty();
        li.sort();
        PermutationIterator { li, is_finished }
    }
}
impl<T: Ord + Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }
        if self.li.len() == 1 {
            self.is_finished = true;
            return Some(self.li.clone());
        }
        let mut i = self.li.len() - 1;
        let res = self.li.clone();
        loop {
            let ii = i;
            i -= 1;
            if self.li[i] < self.li[ii] {
                let mut j = self.li.len() - 1;
                while self.li[i] >= self.li[j] {
                    j -= 1;
                }
                self.li.swap(i, j);
                self.li[ii..].reverse();
                return Some(res);
            }
            if i == 0 {
                self.li.reverse();
                self.is_finished = true;
                return Some(res);
            }
        }
    }
}
pub trait Permutation<T: Ord + Clone> {
    fn permutation(self) -> PermutationIterator<T>;
}
impl<T: Ord + Clone, I: IntoIterator<Item = T>> Permutation<T> for I {
    fn permutation(self) -> PermutationIterator<T> {
        PermutationIterator::new(self.into_iter().collect())
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }

    for p in s.permutation() {
        let ok = (0..(n-1)).all(|i| 
            (0..m).filter(|j| p[i][*j] != p[i+1][*j]).count() <= 1
        );
        if ok {
            println!("Yes");
            return
        }
    }

    println!("No");
}
