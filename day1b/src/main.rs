use itertools::Itertools;

fn main() {
    let input = aoc::input_str(file!());

    let mut values: Vec<i32> = input
        .split("\n\n")
        .map(|chunks| {
            chunks
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect_vec();

    values.sort();

    let sol: i32 = values.into_iter().rev().take(3).sum();

    println!("{}", sol);
}
