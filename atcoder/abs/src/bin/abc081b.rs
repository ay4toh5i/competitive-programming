use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    input! {
        a: [i32; n],
    }

    let result = a.iter().map(|num| num.trailing_zeros()).min().unwrap();

    println!("{}", result);
}
