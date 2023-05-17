use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut result: Vec<u32> = Vec::new();

    a.iter().enumerate().for_each(|(i, current)| {
        if i + 1 >= a.len() {
            result.push(*current);
            return;
        }

        let next = a[i+1];

        if (next > *current && next - *current == 1) || (*current > next && *current - next == 1) {
            result.push(*current);
            return;
        }

        let mut arr = if next > *current { 
            (*current..next).collect::<Vec<u32>>() 
        } else {
            (next+1..=*current).rev().collect::<Vec<u32>>()
        };

        result.append(&mut arr);
    });

    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
