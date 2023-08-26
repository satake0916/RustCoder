use proconio::*;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        query: [(usize, usize); q]
    }

    a.push(1000_000_001);

    // rui[x]: 時刻a[x]までの睡眠時間
    let mut rui = vec![0_usize; a.len()];

    for (i, _x) in a.iter().enumerate().skip(1) {
        if i % 2 == 1 {
            rui[i] = rui[i-1];
        }else{
            rui[i] = rui[i-1] + a[i] - a[i-1];
        }
    }

    let get = |time: &usize| -> usize {
        let idx = a.upper_bound(time);
        rui[idx-1] + (rui[idx] - rui[idx-1]) / (a[idx] - a[idx-1]) * (time - a[idx-1])
    };

    for (l, r) in query {
        println!("{}", get(&r) - get(&l));
    }
}
