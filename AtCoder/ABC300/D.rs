
use proconio::{marker::*, source::line::LineSource};
use proconio::*;
use superslice::Ext;
use std::io::BufReader;
use std::{
    cmp::{*},
    collections::*,
};

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize
    }

    let mut primes = vec![];
    enumerate_prime(1000_000, |x| primes.push(x));

    let mut ans = 0;

    for (j, b) in primes.iter().enumerate().take_while(|p| p.1 * p.1 * p.1 < n) {
        for (i, a) in primes[..j].iter().enumerate().take_while(|p| p.1 * p.1 * b * b * b < n) {
            let c_max = n / a / a / b;
            ans += primes[(j+1)..].upper_bound_by_key(&c_max, |p| p * p);
        }
    }

    println!("{}", ans);
}

// ---------- begin enumerate prime ----------
fn enumerate_prime<F>(n: usize, mut f: F)
where
    F: FnMut(usize),
{
    assert!(1 <= n && n <= 5 * 10usize.pow(8));
    let batch = (n as f64).sqrt().ceil() as usize;
    let mut is_prime = vec![true; batch + 1];
    for i in (2..).take_while(|p| p * p <= batch) {
        if is_prime[i] {
            let mut j = i * i;
            while let Some(p) = is_prime.get_mut(j) {
                *p = false;
                j += i;
            }
        }
    }
    let mut prime = vec![];
    for (i, p) in is_prime.iter().enumerate().skip(2) {
        if *p && i <= n {
            f(i);
            prime.push(i);
        }
    }
    let mut l = batch + 1;
    while l <= n {
        let r = std::cmp::min(l + batch, n + 1);
        is_prime.clear();
        is_prime.resize(r - l, true);
        for &p in prime.iter() {
            let mut j = (l + p - 1) / p * p - l;
            while let Some(is_prime) = is_prime.get_mut(j) {
                *is_prime = false;
                j += p;
            }
        }
        for (i, _) in is_prime.iter().enumerate().filter(|p| *p.1) {
            f(i + l);
        }
        l += batch;
    }
}
// ---------- end enumerate prime ----------
