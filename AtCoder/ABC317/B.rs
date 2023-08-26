use proconio::*;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n]
  }

  a.sort();

  for w in a.windows(2) {
    if w[1] != w[0] + 1 {
        println!("{}", w[0] + 1);
        return
    }
  }
}