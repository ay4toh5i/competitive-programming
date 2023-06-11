use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n < 2 {
        println!("{}", n);
        return;
    }

    let mut result: usize = 2;

    loop {
        if result * 2 > n {
            println!("{}", result);
            return;
        }

        result *= 2;
    }
}
