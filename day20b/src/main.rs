use itertools::Itertools;

fn main() {
    let mut encrypted = aoc::input_lines(file!())
        .map(|s| s.parse::<i64>().unwrap() * 811589153)
        .enumerate()
        .collect_vec();

    let n = encrypted.len();

    for i in (0..n).cycle().take(10 * n) {
        let pos = encrypted.iter().find_position(|v| v.0 == i).unwrap().0;
        let mut v = encrypted[pos].1;

        let m = (n - 1) as i64;
        v = ((v % m) + m) % m;

        for i in 0..v {
            encrypted.swap(
                (pos + i as usize) % n as usize,
                (pos + i as usize + 1) % n as usize,
            );
        }
    }

    let p = encrypted.iter().find_position(|v| v.1 == 0).unwrap().0;

    let answer: i64 = [1000, 2000, 3000]
        .iter()
        .map(|x| encrypted[(x + p) % n].1)
        .sum();

    println!("{}", answer);
}
