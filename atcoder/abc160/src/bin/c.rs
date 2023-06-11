use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n],
    };

    let mut distances: Vec<usize> = vec![];

    for i in 0..(a.len() - 1) {
        distances.push(a[i + 1] - a[i]);
    }

    distances.push(a[0] + k - a[a.len() - 1]);

    let result = distances.iter().sum::<usize>() - distances.iter().max().unwrap();

    println!("{}", result);
}
