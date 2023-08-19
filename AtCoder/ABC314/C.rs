use im_rc::HashSet;
use itertools::Itertools;
use proconio::*;
use proconio::marker::{Chars, Usize1};

fn main() {
  input! {
    n: usize,
    m: usize,
    s: Chars,
    c: [Usize1; n]
  }
  let mut color_vec = vec![vec![]; m];
  for i in 0..n {
    color_vec[c[i]].push(s[i]);
  }
  color_vec.iter_mut().for_each(|v| v.rotate_right(1));
  let mut count = vec![0; m];
  let mut ans = vec![];
  for c in c {
    ans.push(color_vec[c][count[c]]);
    count[c] += 1;
  }

  println!("{}", ans.iter().join(""));
}