use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn input() -> BufReader<File> {
    let day = std::env::var("CARGO_PKG_NAME").unwrap();
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let path = Path::new(&day).join(filename);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
}

fn main() {
    let mut rocks = HashSet::new();
    let mut lower_threshold = 0;

    for line in input().lines().map(|x| x.unwrap()) {
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

    lower_threshold += 2;

    for x in -2 * lower_threshold..2 * lower_threshold {
        rocks.insert((x + 500, lower_threshold));
    }

    let mut iterations = 0;

    while !rocks.contains(&(500, 0)) {
        let mut x = 500;
        let mut y = 0;

        'fall: loop {
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

        iterations += 1;
        rocks.insert((x, y));
    }

    println!("{}", iterations);
}
