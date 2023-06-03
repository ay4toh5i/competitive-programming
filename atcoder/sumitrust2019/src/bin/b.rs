use proconio::input;

// https://img.atcoder.jp/sumitrust2019/editorial.pdf
fn main() {
    input! {
        n: usize
    };

    let x = ((n as f64) / 1.08).ceil();

    if (x * 1.08).floor() as usize == n {
        println!("{}", x);
    } else {
        println!(":(");
    }
}
