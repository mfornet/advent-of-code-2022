use std::collections::HashSet;

fn main() {
    let sol: i32 = aoc::input_lines(file!())
        .map(|line| {
            let values = line
                .chars()
                .map(|x| {
                    if x as i32 >= 'A' as i32 && x as i32 <= 'Z' as i32 {
                        x as i32 - 'A' as i32 + 27
                    } else {
                        x as i32 - 'a' as i32 + 1
                    }
                })
                .collect::<Vec<_>>();
            let n = values.len();

            let h1 = values[..n / 2].iter().copied().collect::<HashSet<_>>();
            let h2 = values[n / 2..].iter().copied().collect::<HashSet<_>>();
            *h1.intersection(&h2).next().unwrap()
        })
        .sum();

    println!("{}", sol);
}
