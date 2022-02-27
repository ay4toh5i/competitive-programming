use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    input! {
        mut a: [i32; n],
    }

    a.sort_by(|x, y| x.cmp(y).reverse());

    let mut alice = 0;
    let mut bob = 0;

    a.iter().enumerate().for_each(|(i, x)| {
        if i % 2 == 0 {
            alice += x;
        } else {
            bob += x;
        }
    });

    println!("{}", alice - bob);
}
