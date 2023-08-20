use im_rc::HashMap;
use proconio::*;

fn main() {
    input! {
      n: usize
    }

    let mut mp = HashMap::new();

    for a in 1..n {
        if a * a >= n {
            break;
        }
        *mp.entry(a * a).or_insert(0_usize) += 1;
        for b in (a + 1)..=(n / a) {
            if a * b == 18 {}
            *mp.entry(a * b).or_insert(0_usize) += 2;
        }
    }

    let mut ans = 0;
    for (k, v) in &mp {
        let rest = n - k;
        ans += v * mp.get(&rest).unwrap_or(&0);
    }

    println!("{}", ans);
}
