use proconio::input;

fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    };

    let mut matrix = vec![vec![false; 3]; 3];

    'outer: for &v in b.iter() {
        for (i, r) in a.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if v == c {
                    matrix[i][j] = true;
                    continue 'outer;
                }
            }
        }
    }

    for x in 0..3 {
        if matrix[x].iter().all(|&v| v) {
            println!("Yes");
            return;
        }
    }

    for x in 0..3 {
        if (0..3).map(|v| matrix[v][x]).all(|v| v) {
            println!("Yes");
            return;
        }
    }

    if (0..3).map(|v| matrix[v][v]).all(|v| v) {
        println!("Yes");
        return;
    }

    if (0..3).map(|v| matrix[v][2 - v]).all(|v| v) {
        println!("Yes");
        return;
    }

    println!("No");
}
