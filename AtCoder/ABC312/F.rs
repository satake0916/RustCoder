use proconio::*;

fn main() {
    input! {
      n: usize,
      m: usize,
      can: [(usize, usize); n]
    }

    let mut tada_vec = vec![];
    let mut ari_vec = vec![];
    let mut ope_vec = vec![];

    for (t, x) in can {
        match t {
            0 => tada_vec.push(x),
            1 => ari_vec.push(x),
            2 => ope_vec.push(x),
            _ => {}
        }
    }

    tada_vec.sort_by(|a, b| b.cmp(a));
    ari_vec.sort();
    ope_vec.sort();

    let mut x = vec![0; n + 1];
    let mut y = vec![0; n + 1];

    for i in 0..n {
        if i < tada_vec.len() {
            x[i + 1] = x[i] + tada_vec[i]
        } else {
            x[i + 1] = x[i]
        }
    }

    let mut rest = 0;
    for i in 0..n {
        if ari_vec.is_empty() {
            y[i + 1] = y[i];
            continue;
        }
        if rest == 0 {
            if let Some(plus) = ope_vec.pop() {
                rest += plus;
            }
            y[i + 1] = y[i];
        } else {
            rest -= 1;
            y[i + 1] = y[i] + ari_vec.pop().unwrap();
        }
    }

    println!("{}", (0..=m).map(|i| x[i] + y[m - i]).max().unwrap());
}
