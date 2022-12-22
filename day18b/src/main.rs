use std::collections::HashSet;

fn main() {
    let mut cubes = HashSet::new();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;

    for line in aoc::input_lines(file!()) {
        let tokens = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let x = tokens[0];
        let y = tokens[1];
        let z = tokens[2];

        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
        min_z = std::cmp::min(min_z, z);
        max_z = std::cmp::max(max_z, z);

        cubes.insert((x, y, z));
    }

    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;
    min_z -= 1;
    max_z += 1;

    let mut answer = 0;

    let mut vis = HashSet::new();
    let source = (min_x, min_y, min_z);
    let mut q = std::collections::VecDeque::new();

    q.push_back(source);
    vis.insert(source);

    while let Some((x, y, z)) = q.pop_front() {
        for (dx, dy, dz) in &[
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let nx = x + dx;
            let ny = y + dy;
            let nz = z + dz;

            if nx < min_x || nx > max_x || ny < min_y || ny > max_y || nz < min_z || nz > max_z {
                continue;
            }

            let node = (nx, ny, nz);

            if vis.contains(&node) {
                continue;
            }

            if cubes.contains(&node) {
                answer += 1;
                continue;
            }

            vis.insert(node);
            q.push_back(node);
        }
    }

    println!("{}", answer);
}
