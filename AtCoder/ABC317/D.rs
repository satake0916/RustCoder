use proconio::*;

fn main() {
  input! {
    n: usize,
    t: [(usize, usize, usize); n]
  }

  let full: usize = t.iter().map(|item| item.2).sum();
  let mut dp = vec![vec![std::usize::MAX / 2; full+1]; n+1];

  let mut needed = vec![];
  for (x, y, z) in t {
    if x > y {
        needed.push((0, z));
    } else {
        needed.push(((x + y + 1) / 2 - x, z));
    }
  }

  dp[0][0] = 0;
  for i in 0..n {
    for j in 0..=full {
        dp[i+1][j] = dp[i+1][j].min(dp[i][j]);
        let nxt = j + needed[i].1;
        if nxt <= full {
            dp[i+1][nxt] = dp[i+1][nxt].min(dp[i][j] + needed[i].0);
        }
    }
  }

  let ans = dp[n].iter().enumerate().filter(|(i, _)| *i > full / 2).map(|(_, value)| *value).min().unwrap();

  println!("{}", ans);
}