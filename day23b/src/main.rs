use std::collections::{HashMap, HashSet};

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1];

fn main() {
    let mut positions = HashSet::new();

    for (x, line) in aoc::input_lines(file!()).enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                positions.insert((x as i32, y as i32));
            }
        }
    }

    let mut step = 0;

    loop {
        let mut proposed_position = HashMap::new();
        let mut proposed_position_freq = HashMap::new();

        for (x, y) in positions.iter().copied() {
            let mut proposal = None;
            let mut has_neighbors = false;

            'neighbors: for rx in -1..=1 {
                for ry in -1..=1 {
                    if rx == 0 && ry == 0 {
                        continue;
                    }

                    let px = x + rx;
                    let py = y + ry;

                    if positions.contains(&(px, py)) {
                        has_neighbors = true;
                        break 'neighbors;
                    }
                }
            }

            if has_neighbors {
                for k in 0..4 {
                    let k = (k + step) % 4;

                    let dx = DX[k];
                    let dy = DY[k];

                    let mut consider = true;

                    'check: for rx in -1..=1 {
                        for ry in -1..=1 {
                            if rx == 0 && ry == 0 {
                                continue;
                            }

                            if (rx - dx).abs() + (ry - dy).abs() > 1 {
                                continue;
                            }

                            let px = x + rx;
                            let py = y + ry;

                            if positions.contains(&(px, py)) {
                                consider = false;
                                break 'check;
                            }
                        }
                    }

                    if consider {
                        proposal = Some((x + dx, y + dy));

                        proposed_position_freq
                            .entry((x + dx, y + dy))
                            .and_modify(|v| *v += 1)
                            .or_insert(1);

                        break;
                    }
                }
            }
            proposed_position.insert((x, y), proposal);
        }

        positions.clear();
        let mut someone_moved = false;

        for (cur, next) in proposed_position.iter() {
            if let Some(next) = next {
                if proposed_position_freq[next] == 1 {
                    someone_moved = true;
                    positions.insert(*next);
                } else {
                    positions.insert(*cur);
                }
            } else {
                positions.insert(*cur);
            }
        }

        step += 1;

        if !someone_moved {
            break;
        }
    }

    println!("{}", step);
}
