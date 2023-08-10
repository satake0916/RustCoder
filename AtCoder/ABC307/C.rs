use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::{cmp::*, collections::*};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MAX: usize = 31;

fn main() {
    input! {
        h_a: usize,
        w_a: usize,
        a: [Chars; h_a],
        h_b: usize,
        w_b: usize,
        b: [Chars; h_b],
        h_x: usize,
        w_x: usize,
        x: [Chars; h_x]
    }

    // fix sheet A position
    for i_a in 0..(MAX - h_a + 1) {
        for j_a in 0..(MAX - w_a + 1) {
            // fix sheet B position
            for i_b in 0..(MAX - h_b + 1) {
                for j_b in 0..(MAX - w_b + 1) {
                    let i_x = 10;
                    let j_x = 10;
                    let mut ok = true;
                    let mut temp = vec![vec!['.'; w_x]; h_x];
                    
                    // as for A
                    for i in 0..h_a {
                        for j in 0..w_a {
                            let i_c = i_a + i;
                            let j_c = j_a + j;
                            let included_x = i_x <= i_c && i_c < i_x + h_x && j_x <= j_c && j_c < j_x + w_x;
                            if included_x && a[i][j] == '#' {
                                temp[i_c - i_x][j_c - j_x] = '#';
                            }
                            if !included_x && a[i][j] == '#' {
                                ok = false;
                            }
                        }
                    }

                    // as for B
                    for i in 0..h_b {
                        for j in 0..w_b {
                            let i_c = i_b + i;
                            let j_c = j_b + j;
                            let included_x = i_x <= i_c && i_c < i_x + h_x && j_x <= j_c && j_c < j_x + w_x;
                            if included_x && b[i][j] == '#' {
                                temp[i_c - i_x][j_c - j_x] = '#';
                            }
                            if !included_x && b[i][j] == '#' {
                                ok = false;
                            }
                        }
                    }

                    // as for X
                    for i in 0..h_x {
                        for j in 0..w_x {
                            ok &= x[i][j] == temp[i][j];
                        }
                    }
                    if ok {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
