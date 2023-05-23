use proconio::input;
use regex::Regex;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    if Regex::new(r"\|\.*\*\.*\|").unwrap().is_match(&s) {
        println!("in");
    } else {
        println!("out");
    }
}
