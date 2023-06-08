use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    };

    let n: usize = format!("{}{}", a ,b).parse().unwrap();

    for x in (0..(n / 2)).rev() {
        if x.pow(2) == n {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
