mod lib; // expand

use lib::*;
use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct P {
    x: i64,
    y: i64,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [P; m],
    }

    // eprintln!("{} {} {:?}", n, m, p);
    println!("0");
}
