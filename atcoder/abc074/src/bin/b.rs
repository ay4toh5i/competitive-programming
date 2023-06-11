use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    };

    let result: usize = x.iter().map(|&v| {
        let d = if k - v < v { k -v } else { v };
        d * 2
    }).sum();

    println!("{}", result);
}
