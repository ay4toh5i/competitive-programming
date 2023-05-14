use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: u32,
        s: Chars,
    }

    let mut t_win = 0;
    let mut a_win = 0;
    let mut t_win_early = false;

    s.iter().for_each(|c| {
        match c {
            'T' => t_win += 1,
            'A' => a_win += 1,
            _ => (),
        }

        if t_win > a_win {
            t_win_early = true;
        }
    });

    if t_win > a_win {
        println!("T");
    } else if a_win > t_win {
        println!("A");
    } else if s.last().unwrap() == &'T' {
        println!("A");
    } else {
        println!("T");
    }
}
