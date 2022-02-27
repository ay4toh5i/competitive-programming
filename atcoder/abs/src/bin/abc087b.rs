use proconio::input;

fn main() {
    input! {
        a: i32,
    }

    input! {
        b: i32,
    }

    input! {
        c: i32,
    }

    input! {
        x: i32,
    }

    let mut ans = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    ans += 1;
                }
            }
        }
    }

    print!("{}", ans);
}
