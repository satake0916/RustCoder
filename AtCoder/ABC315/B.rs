use proconio::*;

fn main() {
  input! {
    m: usize,
    d: [usize; m]
  }

  let mut t: usize = d.iter().sum();
  t /= 2;
  t += 1;

  for (i, cur) in d.iter().enumerate() {
    if *cur >= t {
        println!("{} {}", i+1, t);
        return;
    } else {
        t -= *cur;
    }
  }
}