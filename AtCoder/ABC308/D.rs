use ascii::AsAsciiStr;
use itertools::Itertools;
use proconio::*;
use proconio::{marker::*, source::line::LineSource};
use std::io::BufReader;
use std::{cmp, iter};
use std::{cmp::*, collections::*};
use superslice::Ext;

const MOD: usize = 998244353;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let snuke = "snuke".as_bytes();
    let mut visited = vec![vec![false; w]; h];
    let mut que = VecDeque::new();
    que.push_back((0_usize, 0_usize, 0_usize));

    while let Some((x, y, i)) = que.pop_front() {
        if visited[x][y] || s[x][y] as u8 != snuke[i] {
            continue;
        }
        visited[x][y] = true;

        if x == h - 1 && y == w - 1 {
            println!("Yes");
            return;
        }

        let next = (i + 1) % 5;
        if x > 0 {
            que.push_back((x - 1, y, next))
        }
        if x < h - 1 {
            que.push_back((x + 1, y, next))
        }
        if y > 0 {
            que.push_back((x, y - 1, next))
        }
        if y < w - 1 {
            que.push_back((x, y + 1, next))
        }
    }

    println!("No")
}
