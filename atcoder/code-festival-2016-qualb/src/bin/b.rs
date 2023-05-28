use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };

    let mut accepted = 0;

    let mut accepted_b = 0;

    for &x in s.iter() {
        if x == 'a' && accepted < a + b {
            accepted += 1;
            println!("Yes");
        } else if x == 'b' && accepted < a + b && accepted_b < b {
            accepted += 1;
            accepted_b += 1;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
