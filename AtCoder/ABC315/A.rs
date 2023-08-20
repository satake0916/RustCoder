use itertools::Itertools;
use proconio::*;
use proconio::marker::*;

fn main() {
  input! {
    s: Chars
  }

  println!("{}", s.iter().filter(|c| "aiuoe".chars().all(|x| x != **c)).join(""));
}