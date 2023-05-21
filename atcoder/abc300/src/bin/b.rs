use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    for i in 0..=h {
        for j in 0..=w {
            if a == shift(&b, i, j) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn shift(map: &Vec<Vec<char>>, v_shift: usize, h_shift: usize) -> Vec<Vec<char>> {
    let _h = map.len();
    let _w = map[0].len();

    let mut result = vec![vec!['x'; _w]; _h];

    for i in 0.._h {
        for j in 0.._w {
            result[i][j] = map[(i + v_shift) % _h][(j + h_shift) % _w];
        }
    }

    result
}
