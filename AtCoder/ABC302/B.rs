use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;

fn main() {
    input! {
        h: i32,
        w: i32,
        s: [String; h]
    }

    let dx: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dy: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    let snuke = ['s', 'n', 'u', 'k', 'e'];

    for i in 0..h {
        for j in 0..w {
            for d in 0..8_usize {
                let x = i + dx[d] * 4;
                let y = j + dy[d] * 4;
                if !(0 <= x && x < h && 0 <= y && y < w) {
                    continue;
                }
                if (0..5).all(|ind: usize| {
                    let x = i + dx[d] * ind as i32;
                    let y = j + dy[d] * ind as i32;
                    s[x as usize].chars().nth(y as usize).unwrap() == snuke[ind]
                }) {
                    (0..5).for_each(|ind|
                        println!("{} {}", i + dx[d] * ind + 1, j + dy[d] * ind + 1)
                    );
                    return
                }
            }
        }
    }
}
