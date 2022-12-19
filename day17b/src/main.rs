use std::{collections::HashSet, io::Read};

fn collides(shape: &Vec<(i32, i32)>, dx: i32, dy: i32, used: &HashSet<(i32, i32)>) -> bool {
    for (x, y) in shape {
        let x = x + dx;
        let y = y + dy;

        if used.contains(&(x, y)) || x < 0 || !(0..7).contains(&y) {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn preview(used: &HashSet<(i32, i32)>) {
    let top = used.iter().map(|x| x.0).max().unwrap();

    for i in 0..=top {
        print!("|");
        for y in 0..7 {
            if used.contains(&(top - i, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
}

fn main() {
    const TARGET_SIZE: u64 = 1_000_000_000_000;
    const INSPECT_PREFIX: usize = 1_000_000;
    const CYCLE_CHECK: usize = 7_500;

    let mut pattern = String::new();
    aoc::input(file!()).read_to_string(&mut pattern).unwrap();
    let pattern = pattern.trim_matches('\n').to_string();
    let mut pattern = pattern.chars().cycle();

    let shapes = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(1, 0), (1, 1), (1, 2), (0, 1), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut used = HashSet::default();
    let mut top = -1;
    let mut tops = vec![];

    for shape in shapes.iter().cycle().take(INSPECT_PREFIX) {
        let prev_top = top;
        let mut dx = top + 4;
        let mut dy = 2;

        loop {
            // Go with the wind
            let s = match pattern.next().unwrap() {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };

            if !collides(shape, dx, dy + s, &used) {
                dy += s;
            }

            // Go down
            if collides(shape, dx - 1, dy, &used) {
                break;
            }
            dx -= 1;
        }

        for (x, y) in shape {
            let x = x + dx;
            let y = y + dy;
            used.insert((x, y));
            top = std::cmp::max(top, x);
        }

        tops.push((top - prev_top) as u64);
    }

    let mut turtle = 0;
    let mut hare = 0;

    loop {
        turtle += 1;
        hare += 2;

        if (0..CYCLE_CHECK).all(|i| tops[turtle + i] == tops[hare + i]) {
            break;
        }
    }

    let mut answer = 0;

    for i in 0..turtle {
        answer += tops[i];
    }

    let cycle_len = hare - turtle;
    let cycle_sum: u64 = tops[turtle..hare].iter().sum();
    let full_cycle_times = (TARGET_SIZE - turtle as u64) / cycle_len as u64;
    let remaining_steps = (TARGET_SIZE - turtle as u64) % cycle_len as u64;

    answer += full_cycle_times * cycle_sum;

    for i in 0..remaining_steps as usize {
        answer += tops[turtle + i];
    }

    println!("{}", answer);
}
