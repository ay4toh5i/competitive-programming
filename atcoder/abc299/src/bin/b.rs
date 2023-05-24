use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let f = c[0];

    let mut t_win: Option<usize> = None;
    let mut f_win: Option<usize> = None;

    for (i, (&a, &b)) in c.iter().zip(&r).enumerate() {
        if a == t && (t_win == None || b > r[t_win.unwrap()]) {
            t_win = Some(i);
        }

        if a == f && (f_win == None || b > r[f_win.unwrap()]) {
            f_win = Some(i);
        }
    }

    if let Some(x) = t_win {
        println!("{}", x  + 1);
        return;
    } 

    if let Some(y) = f_win {
        println!("{}", y + 1);
    }
}
