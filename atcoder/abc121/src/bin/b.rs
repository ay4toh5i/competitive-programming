use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: isize,
        b: [isize; m],
        a: [[isize; m]; n],
    };

    let result = a.iter().filter(|&x| {
        let s = x.iter().enumerate().map(|(i, &y)| y * b[i]).sum::<isize>() + c;

        s > 0
    }).count();

    println!("{}", result);
}
