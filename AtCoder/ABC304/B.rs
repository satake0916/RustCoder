use itertools::Itertools;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::convert::TryInto;

fn main() {
    input! {
        n: u32
    }

    if n < 10_u32.pow(3) {
        println!("{}", n);
    }else{
        let digit: u32 = n.to_string().len().try_into().unwrap();
        println!("{}", n / 10_u32.pow(digit - 3_u32) * 10_u32.pow(digit - 3_u32));
    }
}
