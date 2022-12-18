use std::collections::HashSet;

fn abs(v: i32) -> i32 {
    if v < 0 {
        -v
    } else {
        v
    }
}

fn main() {
    let target = 2_000_000;
    // let target = 10;

    let mut events = vec![];
    let mut beacons = HashSet::new();

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

        beacons.insert((x1, y1));

        let d = abs(x0 - x1) + abs(y0 - y1);
        let dy = abs(y0 - target);

        if d >= dy {
            let diff = d - dy;
            events.push((x0 - diff, 1));
            events.push((x0 + diff + 1, -1));
        }
    }

    let mut total = 0;

    for (_, y) in beacons {
        if y == target {
            total -= 1;
        }
    }

    events.sort();

    let ini = events[0].0;
    let fin = events.last().unwrap().0;
    let mut pnt = 0;
    let mut open = 0;

    for x in ini..fin {
        if events[pnt].0 == x {
            open += events[pnt].1;
            pnt += 1;
        }

        if open >= 1 {
            total += 1;
        }
    }

    println!("{}", total);
}
