use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut r = 0;
    let mut f = 0;

    for &x in s.iter() {
        if x == 'o' {
            r += 1;
        }

        if x == 'x' {
            f += 1;
        }
    }

    if f == 0 && r > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
