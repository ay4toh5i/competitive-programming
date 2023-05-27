use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    // コンセントをb-1増やす、電源タップひとつでa-1個増やせる
    // (b-1) / (a-1) の切り上げ
    println!("{}", (b - 1 + a - 2) / (a - 1));
}
