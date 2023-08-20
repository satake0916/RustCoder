use std::collections::BinaryHeap;

use proconio::{*, marker::Usize1};

fn main() {
  input! {
    n: usize,
    ice: [(Usize1, usize); n]
  }

  let mut aji = vec![BinaryHeap::<usize>::new(); n];
  let mut tuyoi = BinaryHeap::<usize>::new();
  let mut ans = 0;

  for (f, s) in ice {
    aji[f].push(s);
  }

  for mut aji in aji {
    if aji.len() >= 1 {
        let first = aji.pop().unwrap();
        tuyoi.push(first);
        if aji.len() >= 1 {
            let second = aji.pop().unwrap();
            ans = ans.max(first + second / 2);
        }
    }
  }

  if tuyoi.len() >= 2 {
    ans = ans.max(tuyoi.pop().unwrap() + tuyoi.pop().unwrap());
  }

  println!("{}", ans);
}