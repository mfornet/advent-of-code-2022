use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let sol: i32 = aoc::input_lines(file!())
        .map(|rucksack| {
            rucksack
                .chars()
                .map(|x| {
                    if x as i32 >= 'A' as i32 && x as i32 <= 'Z' as i32 {
                        x as i32 - 'A' as i32 + 27
                    } else {
                        x as i32 - 'a' as i32 + 1
                    }
                })
                .collect::<Vec<_>>()
        })
        .chunks(3)
        .into_iter()
        .map(|team| {
            team.map(|rucksack| rucksack.into_iter().collect::<HashSet<_>>())
                .fold(Option::<HashSet<_>>::None, |acc, x| {
                    if let Some(acc) = acc {
                        Some(acc.intersection(&x).copied().collect::<HashSet<_>>())
                    } else {
                        Some(x)
                    }
                })
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        })
        .sum();

    println!("{}", sol);
}
