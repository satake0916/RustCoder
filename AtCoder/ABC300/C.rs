use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::{*, self}, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        mut c: [Chars; h]
    }

    for c in c.iter_mut() {
        c.push('.');
        c.insert(0, '.');
    }
    c.insert(0, vec!['.'; w+2]);
    c.push(vec!['.'; w+2]);

    let n = cmp::min(h, w);

    let mut ans = vec![0; n+1];

    let mut sol = |i: usize, j: usize| {
        if c[i][j] != '#' {
            return
        }
        for cnt in 1..=n {
            if c[i + cnt][j + cnt] == '#' && c[i + cnt][j - cnt] == '#' && c[i - cnt][j + cnt] == '#' && c[i - cnt][j - cnt] == '#' {
                continue;
            }else{
                ans[cnt-1] += 1;
                break;
            }
        }
        return
    };

    for i in 0..h {
        for j in 0..w {
            sol(i+1, j+1);
        }
    }


    println!("{}", ans.iter().skip(1).join(" "));
}
