use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize
    }
    let mut person = vec![];

    for _ in 0..n {
        input! {
            c: usize,
            a: [usize; c]
        }
        person.push((c, a.into_iter().collect::<BTreeSet<usize>>()));
    }
    input! { x: usize }

    let min_value = person.iter().filter(|p| (*p).1.contains(&x)).min_by_key(|p| p.0);
    let mins = match min_value {
        Some((min_c, _min_set)) => person.iter().enumerate().filter(|(_, p)| p.0 == *min_c && p.1.contains(&x)).map(|(i, _)| i+1).collect_vec(),
        None => vec![]
    };
    println!("{}\n{}", mins.len(), mins.iter().join(" "));
}