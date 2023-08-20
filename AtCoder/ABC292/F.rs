use proconio::*;
use std::f64::consts::PI;

fn main() {
  input! {
    mut a: usize,
    mut b: usize
  }

  if a > b {
    let t = a;
    a = b;
    b = t;
  }

  // a <= b になった

  // 十分大きいのでbにベタッとつけていい
  let saidai = a as f64 * 2_f64 * 3_f64.sqrt() / 3_f64;
  if b as f64 >= saidai {
        println!("{:.15}", saidai);
        return;
  }

  let cos30 = 3_f64.sqrt() / 2_f64;
  let sin30 = 0.5;

  let need_len = |theta: f64| -> f64 {
    let rad = theta/180_f64 * PI;
    a as f64 * cos30 + a as f64 * sin30 * rad.tan()
  };

  let mut r = 30_f64;// 絶対無理
  let mut l = 15_f64;// 絶対いける

  while need_len(r) - need_len(l) > 0.000_000_00001 {
    let mid = (r + l) / 2_f64;
    let res = need_len(mid);
    if res <= b as f64 {
        l = mid;
    } else {
        r = mid;
    }
  }

  // println!("{l}");

  let rad = l / 180_f64 * PI;
  let ans = a as f64 / rad.cos();

  println!("{:.15}", ans);
}