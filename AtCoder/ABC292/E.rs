use std::collections::VecDeque;

use im_rc::HashSet;
use proconio::{*, marker::Usize1};

fn main() {
  input! {
    n: usize,
    m: usize,
    e: [(Usize1, Usize1); m]
  }

  let mut edges = vec![vec![]; n];
  for (u, v) in e {
    edges[u].push(v);
  }

  let mut fin = 0;

  for start in 0..n {
    let mut st = HashSet::new();
    let mut que = VecDeque::new();
    st.insert(start);
    que.push_back(start);

    while let Some(top) = que.pop_front() {
        st.insert(top);
        for next in &edges[top] {
            if !st.contains(next) {
                que.push_back(*next)
            }
        }
    }

    fin += st.len() - 1;
  }

  println!("{}", fin - m);
}