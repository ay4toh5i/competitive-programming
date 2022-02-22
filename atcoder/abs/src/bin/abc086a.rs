use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    match a * b % 2 {
        0 => println!("Even"),
        1 => println!("Odd"),
        _ => unreachable!(),
    }
}
