use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let res = s
        .replace("eraser", "")
        .replace("erase", "")
        .replace("dreamer", "")
        .replace("dream", "");

    if res.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
