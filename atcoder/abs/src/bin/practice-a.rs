use proconio::input;

fn main() {
    input! {
        a: i32,
    }

    input! {
        b: i32,
        c: i32,
    }

    input! {
        s: String,
    }

    println!("{} {}", a + b + c, s);
}
