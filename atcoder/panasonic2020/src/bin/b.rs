use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    let even_col_point = h / 2;
    let odd_col_point = h - even_col_point;

    let even_col_num = w / 2;
    let odd_col_num = w - even_col_num;

    println!("{}", even_col_point * even_col_num + odd_col_point * odd_col_num);
}
