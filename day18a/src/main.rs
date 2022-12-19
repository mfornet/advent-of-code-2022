use std::collections::HashSet;

fn main() {
    let mut cubes = HashSet::new();

    for line in aoc::input_lines(file!()) {
        let tokens = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let x = tokens[0];
        let y = tokens[1];
        let z = tokens[2];

        cubes.insert((x, y, z));
    }

    let mut answer = cubes.len() * 6;

    for (x, y, z) in cubes.iter().copied() {
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

            if cubes.contains(&(nx, ny, nz)) {
                answer -= 1;
            }
        }
    }

    println!("{}", answer);
}
