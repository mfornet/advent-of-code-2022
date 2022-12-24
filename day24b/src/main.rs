use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Stage {
    First,
    Second,
    Third,
}

impl From<Stage> for i32 {
    fn from(stage: Stage) -> Self {
        match stage {
            Stage::First => 0,
            Stage::Second => 1,
            Stage::Third => 2,
        }
    }
}

impl PartialOrd for Stage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Stage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let u: i32 = (*self).into();
        let v: i32 = (*other).into();
        u.cmp(&v)
    }
}

fn main() {
    let board = aoc::input_lines(file!()).collect_vec();
    let mut blizzards = HashSet::new();

    const DX: [i32; 5] = [0, 1, 0, -1, 0];
    const DY: [i32; 5] = [1, 0, -1, 0, 0];
    const DIR: &str = ">v<^";

    let height = board.len() as i32;
    let width = board[0].len() as i32;

    let start = (
        0,
        board[0].chars().find_position(|c| *c == '.').unwrap().0 as i32,
    );
    let end = (
        (height - 1) as i32,
        board
            .last()
            .unwrap()
            .chars()
            .find_position(|c| *c == '.')
            .unwrap()
            .0 as i32,
    );

    for (row, line) in aoc::input_lines(file!()).enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(d) = DIR.find(c) {
                blizzards.insert((row, col, d));
            }
        }
    }

    let mut visited = HashSet::new();

    let start_end_dist = (start.0 - end.0).abs() + (start.1 - end.1).abs();

    let heuristic = |(x, y, stage, t): &(i32, i32, Stage, i32)| {
        t + match stage {
            Stage::First => (x - end.0).abs() + (y - end.1).abs() + 2 * start_end_dist,
            Stage::Second => (x - start.0).abs() + (y - start.1).abs() + start_end_dist,
            Stage::Third => (x - end.0).abs() + (y - end.1).abs(),
        }
    };
    let mut queue = BinaryHeap::new();

    visited.insert((start.0, start.1, Stage::First, 0));

    queue.push((
        -heuristic(&(start.0, start.1, Stage::First, 0)),
        start.0,
        start.1,
        Stage::First,
        0,
    ));

    while let Some((_, x, y, stage, t)) = queue.pop() {
        'check_step: for d in 0..5 {
            let nx = x as i32 + DX[d];
            let ny = y as i32 + DY[d];
            let nt = t + 1;
            let ns = match stage {
                Stage::First => {
                    if nx == end.0 && ny == end.1 {
                        Stage::Second
                    } else {
                        Stage::First
                    }
                }
                Stage::Second => {
                    if nx == start.0 && ny == start.1 {
                        Stage::Third
                    } else {
                        Stage::Second
                    }
                }
                Stage::Third => Stage::Third,
            };

            if nx == end.0 && ny == end.1 && stage == Stage::Third {
                println!("{}", nt);
                return;
            }

            if nx < 0
                || ny < 0
                || nx >= height
                || ny >= width
                || board[nx as usize].chars().nth(ny as usize).unwrap() == '#'
                || visited.contains(&(nx, ny, ns, nt))
            {
                continue;
            }

            for (bx, by, bd) in blizzards.iter() {
                let mut bx = *bx as i32 - 1 + DX[*bd] * nt as i32;
                bx = (bx % (height - 2) + (height - 2)) % (height - 2) + 1;

                let mut by = *by as i32 - 1 + DY[*bd] * nt as i32;
                by = (by % (width - 2) + (width - 2)) % (width - 2) + 1;

                if nx == bx && ny == by {
                    continue 'check_step;
                }
            }

            visited.insert((nx, ny, ns, nt));
            queue.push((-heuristic(&(nx, ny, ns, nt)), nx, ny, ns, nt));
        }
    }
}
