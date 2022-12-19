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

    for shape in shapes.iter().cycle().take(2022) {
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
    }

    println!("{}", top + 1);
}
