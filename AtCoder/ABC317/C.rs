use proconio::{*, marker::Usize1};

fn rec(visited: usize, cost: usize, cur: usize, edges: &Vec<Vec<(usize, usize)>>) -> usize {
    edges[cur].iter().filter(|next| visited & (1 << next.0) == 0).map(
        |next| rec(visited + (1 << next.0), cost + next.1, next.0, edges)
    ).max().unwrap_or(cost)
}

fn main() {
  input! {
    n: usize,
    m: usize,
    t: [(Usize1, Usize1, usize); m]
  }

  let mut edges = vec![vec![]; n];
  for (a, b, c) in t {
    edges[a].push((b, c));
    edges[b].push((a, c));
  }

  let mut ans = 0;

  for start in 0..n {
    let visited = 1 << start;
    ans = ans.max(rec(visited, 0, start, &edges));
  }

  println!("{}", ans);
}