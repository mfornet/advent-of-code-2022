use std::collections::HashSet;

fn abs(v: i32) -> i32 {
    if v < 0 {
        -v
    } else {
        v
    }
}

fn main() {
    let mut sensors = HashSet::new();
    let pat = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();
    for line in aoc::input_lines(file!()) {
        let cap = pat.captures(&line).unwrap();

        let x0 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y0 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let x1 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let y1 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();

        let d = abs(x0 - x1) + abs(y0 - y1);
        sensors.insert((x0, y0, d));
    }

    let big = 4_000_000;
    let target = 4_000_000;
    // let target = 20;

    for yt in 0..=target {
        let mut events = vec![];

        for (x, y, d) in sensors.iter() {
            let dy = abs(y - yt);

            if *d >= dy {
                let dd = *d - dy;
                events.push((*x - dd, -1));
                events.push((*x + dd + 1, 1));
            }
        }

        events.sort();

        let mut open = 0;

        for (x, d) in events {
            open -= d;

            if 0 <= x && x <= target && open == 0 {
                println!("{} ({}, {})", x as i64 * big as i64 + yt as i64, x, yt);
            }
        }
    }
}
