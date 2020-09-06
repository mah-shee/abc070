use num_integer::lcm;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let mut ans = 1;
    for &i in t.iter() {
        ans = lcm(ans, i);
    }
    println!("{}", ans);
}
