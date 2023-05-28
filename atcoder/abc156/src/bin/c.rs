use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    };

    let &max = x.iter().max().unwrap();
    let &min = x.iter().min().unwrap();

    let result = (min..=max)
        .map(|i| x.iter().map(|&v| abs_diff(v, i).pow(2)).sum::<usize>())
        .min()
        .unwrap();

    println!("{}", result);
}

fn abs_diff(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}
