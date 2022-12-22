use itertools::Itertools;

#[derive(PartialEq)]
enum Cell {
    Ground,
    Wall,
    Empty,
}

#[derive(Debug)]
enum Action {
    Rotate(usize),
    Move(i32),
}

struct Actions<'a>(&'a str);

impl<'a> Iterator for Actions<'a> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .chars()
            .find_position(|c| !c.is_ascii_digit())
            .map(|(i, _)| {
                if i > 0 {
                    let (num, rest) = self.0.split_at(i);
                    self.0 = rest;
                    Action::Move(num.parse().unwrap())
                } else {
                    let (action, rest) = self.0.split_at(1);
                    self.0 = rest;

                    match action {
                        "R" => Action::Rotate(1),
                        "L" => Action::Rotate(3),
                        _ => panic!("Unknown action: {}", action),
                    }
                }
            })
            .or_else(|| {
                if !self.0.is_empty() {
                    let num = self.0.parse().unwrap();
                    self.0 = "";
                    Some(Action::Move(num))
                } else {
                    None
                }
            })
    }
}

fn main() {
    let mut map = Vec::new();

    let mut commands = String::new();
    let mut width = 0;

    for line in aoc::input_lines(file!()) {
        if !line.is_empty() && line.chars().next().unwrap().is_ascii_digit() {
            commands = line;
        } else {
            map.push(
                line.chars()
                    .map(|c| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Ground,
                        ' ' => Cell::Empty,
                        _ => panic!("Unknown cell type: {}", c),
                    })
                    .collect_vec(),
            );
            width = std::cmp::max(width, line.len() as i32);
        }
    }

    let length = map.len() as i32;

    for row in map.iter_mut() {
        row.resize_with(width as usize, || Cell::Empty);
    }

    let mut y = map[0]
        .iter()
        .find_position(|p| **p == Cell::Ground)
        .unwrap()
        .0 as i32;

    let mut x = 0;
    let mut d = 0;

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    for action in Actions(&commands) {
        match action {
            Action::Rotate(dir) => {
                d = (d + dir) & 3;
            }
            Action::Move(steps) => {
                for _ in 0..steps {
                    let mut nx = (x + dx[d] + length) % length;
                    let mut ny = (y + dy[d] + width) % width;

                    while map[nx as usize][ny as usize] == Cell::Empty {
                        nx = (nx + dx[d] + length) % length;
                        ny = (ny + dy[d] + width) % width;
                    }

                    if map[nx as usize][ny as usize] == Cell::Wall {
                        break;
                    } else {
                        x = nx;
                        y = ny;
                    }
                }
            }
        }
    }

    println!("{}", (x + 1) * 1000 + (y + 1) * 4 + d as i32);
}
