use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let input = aoc::input(file!());

    let mut rocks = HashSet::new();
    let mut lower_threshold = 0;

    for line in input.lines().map(|x| x.unwrap()) {
        line.split(" -> ")
            .map(|pos| {
                let (x, y) = pos.split_at(pos.find(',').unwrap());
                let x = x.parse::<i32>().unwrap();
                let y = y[1..].parse::<i32>().unwrap();
                lower_threshold = std::cmp::max(lower_threshold, y);
                (x, y)
            })
            .collect_vec()
            .windows(2)
            .for_each(|segment| {
                let start = segment[0];
                let fin = segment[1];

                let (mut x1, mut y1) = start;
                let (mut x2, mut y2) = fin;

                if x1 > x2 {
                    std::mem::swap(&mut x1, &mut x2);
                }

                if y1 > y2 {
                    std::mem::swap(&mut y1, &mut y2);
                }

                for x in x1..=x2 {
                    for y in y1..=y2 {
                        rocks.insert((x, y));
                    }
                }
            });
    }

    lower_threshold += 1;
    let mut iterations = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        'fall: while y < lower_threshold {
            for dx in [0, -1, 1] {
                let nx = x + dx;
                let ny = y + 1;

                if !rocks.contains(&(nx, ny)) {
                    x = nx;
                    y = ny;
                    continue 'fall;
                }
            }

            break;
        }

        if y < lower_threshold {
            iterations += 1;
            rocks.insert((x, y));
        } else {
            break;
        }
    }

    println!("{}", iterations);
}
