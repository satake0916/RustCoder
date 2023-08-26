use std::collections::VecDeque;

use proconio::{*, marker::Chars};

fn main() {
  input! {
    h: usize,
    w: usize,
    area: [Chars; h]
  }

  let mut need = vec![vec![std::usize::MAX / 2; w]; h];
  let mut deque = VecDeque::new();

  for i in 0..h {
    for j in 0..w {
      if area[i][j] == 'S' {
        need[i][j] = 0;
        deque.push_back((i, j));
      }
    }
  }

  let mut can_visit = vec![vec![true; w]; h];
  for i in 0..h {
    for j in 0..w {
      let mut ti = i;
      let mut tj = j;
      can_visit[i][j] &= match area[i][j] {
        '#' => false,
        'v' => {
          while ti + 1 < h && area[ti + 1][j] == '.' {
            can_visit[ti + 1][j] = false;
            ti += 1;
          }
          false
        },
        '>' => {
          while tj + 1 < w && area[i][tj + 1] == '.' {
            can_visit[i][tj + 1] = false;
            tj += 1;
          }
          false
        },
        '^' => {
          while ti > 0 && area[ti - 1][j] == '.' {
            can_visit[ti - 1][j] = false;
            ti -= 1;
          }
          false
        },
        '<' => {
          while tj > 0 && area[i][tj - 1] == '.' {
            can_visit[i][tj - 1] = false;
            tj -= 1;
          }
          false
        },
        _ => true
      }
    }
  }

  // println!("{:?}", can_visit);

  let dif = [(0_usize, 1_usize), (0, !0), (1, 0), (!0, 0)];

  while let Some((i, j)) = deque.pop_front() {
    let cur_cost = need[i][j];
    if area[i][j] == 'G' {
      println!("{}", cur_cost);
      return
    }
    for (dx, dy) in dif {
      let nx = i.wrapping_add(dx);
      let ny = j.wrapping_add(dy);

      // 枠からはみ出していないか
      if nx >= h || ny >= w {
        continue;
      }

      // 進入可能か
      if !can_visit[nx][ny] {
        continue;
      }

      // すでに通り過ぎていないか
      if need[nx][ny] <= cur_cost + 1 {
        continue;
      }

      // 進んでみる
      need[nx][ny] = cur_cost + 1;
      deque.push_back((nx, ny));
    }
  }

  println!("-1");
}