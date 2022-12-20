use itertools::Itertools;

fn main() {
    let mut encrypted = aoc::input_lines(file!())
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .collect_vec();

    let n = encrypted.len();

    for i in 0..n {
        let mut pos = encrypted.iter().find_position(|v| v.0 == i).unwrap().0;

        let mut v = encrypted[pos].1;

        let s = v < 0;

        if s {
            encrypted.reverse();
            v = -v;
            pos = n - 1 - pos;
        }

        for _ in 0..v {
            if pos == n - 1 {
                encrypted.swap(pos, 0);
            } else {
                encrypted.swap(pos, pos + 1);
            }

            pos = (pos + 1) % n;
        }

        if s {
            encrypted.reverse();
        }
    }

    let p = encrypted.iter().find_position(|v| v.1 == 0).unwrap().0;

    let answer: i32 = [1000, 2000, 3000]
        .iter()
        .map(|x| encrypted[(x + p) % n].1)
        .sum();

    println!("{}", answer);
}
