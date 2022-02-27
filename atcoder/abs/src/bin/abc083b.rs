use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let ans = (0..=n)
        .filter(|n| {
            let sum = sum_of_digits(*n);
            a <= sum && sum <= b
        })
        .sum::<i32>();

    println!("{}", ans);
}

fn sum_of_digits(n: i32) -> i32 {
    let mut num = n;
    let mut sum = 0;

    while num > 0 {
        sum += num % 10;
        num /= 10;
    }

    sum
}
