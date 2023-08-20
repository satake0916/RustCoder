use proconio::*;

fn main() {
    input! {
      n: usize,
      q: usize,
      event: [(usize, usize); q]
    }

    let mut card = vec![0_usize; n + 1];

    for (op, x) in event {
        match op {
            1 => card[x] += 1,
            2 => card[x] += 2,
            3 => println!("{}", if card[x] >= 2 { "Yes" } else { "No" }),
            _ => {}
        }
    }
}
