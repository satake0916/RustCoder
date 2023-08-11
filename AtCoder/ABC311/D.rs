use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::{*, self}, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }
    
    let mut visited = vec![vec![vec![false; 4]; m]; n];
    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, 1, -1];
    let mut q = Deque::new();
    for i in 0..4 { q.push_back((1, 1, i)) }

    while !q.is_empty() {
        let (mut x, mut y, dir) = q.pop_front().unwrap();
        if visited[x][y][dir] {
            continue;
        }

        visited[x][y][dir] = true;

        while s[(x as i32 + dx[dir]) as usize][(y as i32 + dy[dir]) as usize] == '.' {
            x = (x as i32 + dx[dir]) as usize;
            y = (y as i32 + dy[dir]) as usize;
            visited[x][y][dir] = true;
        }
        q.push_back((x, y, (dir + 1) % 4));
        q.push_back((x, y, (dir + 3) % 4));
    }

    println!("{}", visited.iter().flatten().filter(|x| x.iter().any(|y| *y)).count())
}
