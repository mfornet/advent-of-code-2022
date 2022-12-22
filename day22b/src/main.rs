use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Clone, Debug, Copy)]
enum Cell {
    Ground,
    Wall,
    Empty,
}

#[derive(Debug, Clone)]
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

const DX: &[i32; 4] = &[0, 1, 0, -1];
const DY: &[i32; 4] = &[1, 0, -1, 0];

#[derive(Debug)]
struct Cube {
    side: i32,
    map: Vec<Vec<Cell>>,
    quads_move: [[(usize, usize); 4]; 6],
    quads_id: HashMap<(i32, i32), usize>,
}

fn quad(x: i32, y: i32, side: i32) -> (i32, i32) {
    ((x + side) / side, (y + side) / side)
}

impl Cube {
    fn new(map: Vec<Vec<Cell>>) -> Self {
        let mut count = map
            .iter()
            .flat_map(|row| row.iter().filter(|cell| cell != &&Cell::Empty))
            .count();

        assert!(count % 6 == 0);
        count /= 6;

        let side = (count as f64).sqrt() as i32;
        assert_eq!(side * side, count as i32);

        let mut quads_move = [[Option::<(usize, usize)>::None; 4]; 6];
        let mut quads_id = HashMap::new();

        for (row_id, row) in map.iter().enumerate() {
            for (col_id, cell) in row.iter().enumerate() {
                if cell != &Cell::Empty {
                    let id = quad(row_id as i32, col_id as i32, side);
                    let n = quads_id.len();
                    quads_id.entry(id).or_insert_with(|| n);
                }
            }
        }

        assert_eq!(quads_id.len(), 6);
        let mut missing = 6 * 4;

        for ((x, y), id) in quads_id.iter() {
            for d in 0..4 {
                let nx = x + DX[d];
                let ny = y + DY[d];

                if let Some(&n_id) = quads_id.get(&(nx, ny)) {
                    quads_move[*id][d] = Some((n_id, d));
                    missing -= 1;
                }
            }
        }

        while missing > 0 {
            for id in 0..6 {
                for d in 0..4 {
                    if quads_move[id][d].is_some() {
                        continue;
                    }

                    let ld = (d + 3) % 4;

                    if let Some((n_id, nd)) = quads_move[id][ld] {
                        let rd = (nd + 1) % 4;

                        if let Some((nn_id, nd)) = quads_move[n_id][rd] {
                            let ld = (nd + 3) % 4;
                            quads_move[id][d] = Some((nn_id, ld));
                            missing -= 1;
                        }
                    }
                }
            }
        }

        let quads_move = {
            let mut target = [[(0, 0); 4]; 6];
            for (src, dst) in quads_move.into_iter().zip(target.iter_mut()) {
                for (src, dst) in src.into_iter().zip(dst.iter_mut()) {
                    *dst = src.unwrap();
                }
            }
            target
        };

        Self {
            side,
            map,
            quads_move,
            quads_id,
        }
    }

    fn quad(&self, x: i32, y: i32) -> (i32, i32) {
        quad(x, y, self.side)
    }

    fn step(&self, x: i32, y: i32, d: usize) -> (i32, i32, usize) {
        let nx = x + DX[d];
        let ny = y + DY[d];

        let q = self.quad(x, y);
        let nq = self.quad(nx, ny);

        if q == nq {
            (nx, ny, d)
        } else {
            let q_id = self.quads_id[&q];
            let (nq_id, nd) = self.quads_move[q_id][d];

            let mut cx = x % self.side;
            let mut cy = y % self.side;

            let mut md = d;
            while md != nd {
                md = (md + 1) % 4;
                let nx = cy;
                let ny = self.side - cx - 1;
                cx = nx;
                cy = ny;
            }

            let (ncx, ncy) = self.quads_id.iter().find(|(_, &id)| id == nq_id).unwrap().0;

            let ncx = (*ncx - 1) * self.side;
            let ncy = (*ncy - 1) * self.side;

            (
                ncx + cx + (1 - self.side) * DX[nd],
                ncy + cy + (1 - self.side) * DY[nd],
                nd,
            )
        }
    }

    fn get(&self, x: i32, y: i32) -> Cell {
        self.map[x as usize][y as usize]
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

    for row in map.iter_mut() {
        row.resize_with(width as usize, || Cell::Empty);
    }

    let mut y = map[0]
        .iter()
        .find_position(|p| **p == Cell::Ground)
        .unwrap()
        .0 as i32;

    let cube = Cube::new(map);

    let mut x = 0;
    let mut d = 0;

    for action in Actions(&commands) {
        match action {
            Action::Rotate(dir) => {
                d = (d + dir) & 3;
            }
            Action::Move(steps) => {
                for _ in 0..steps {
                    let (nx, ny, nd) = cube.step(x, y, d);
                    match cube.get(nx, ny) {
                        Cell::Ground => {
                            x = nx;
                            y = ny;
                            d = nd;
                        }
                        Cell::Wall => break,
                        Cell::Empty => panic!("Unexpected empty cell"),
                    }
                }
            }
        }
    }

    println!("{}", (x + 1) * 1000 + (y + 1) * 4 + d as i32);
}
