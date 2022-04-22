use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    println!("{}", a.into_iter().collect::<HashSet<i32>>().len());
}
