use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    };

    let mut _a = a;

    for _x in 0..4 {
        let mut ok = true;

        for i in 0.._a.len() {
            for j in 0.._a.len() {
                if _a[i][j] == 1 && b[i][j] == 0 {
                    ok = false;
                }
            }
        }

        if ok {
            println!("Yes");
            return;
        }
        
        _a = rot(_a);
    }

    println!("No");
}

fn rot(m: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let size = m.len();
    let mut result = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            result[i][j] = m[size-j-1][i];
        }
    }

    result
}
