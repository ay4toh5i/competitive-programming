use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
        answers: [u32; n],
    }

    let sum = a + b;

    let index = answers.iter().position(|&x| x == sum).unwrap();

    println!("{}", index + 1);
}
