use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

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

    let heuristic = |(x, y, t): &(i32, i32, i32)| (x - end.0).abs() + (y - end.1).abs() + t;
    let mut queue = BinaryHeap::new();

    visited.insert((start.0, start.1, 0));
    queue.push((-heuristic(&(start.0, start.1, 0)), start.0, start.1, 0));

    while let Some((_, x, y, t)) = queue.pop() {
        'check_step: for d in 0..5 {
            let nx = x as i32 + DX[d];
            let ny = y as i32 + DY[d];
            let nt = t + 1;

            if nx == end.0 && ny == end.1 {
                println!("{}", nt);
                return;
            }

            if nx < 0
                || ny < 0
                || nx >= height
                || ny >= width
                || board[nx as usize].chars().nth(ny as usize).unwrap() == '#'
                || visited.contains(&(nx, ny, nt))
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

            visited.insert((nx, ny, nt));
            queue.push((-heuristic(&(nx, ny, nt)), nx, ny, nt));
        }
    }
}
