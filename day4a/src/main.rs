use itertools::Itertools;

fn main() {
    let sol = aoc::input_lines(file!())
        .map(|pair| {
            let mut it = pair.split(',').map(|s| {
                if let [a, b] = s
                    .split('-')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_vec()[..]
                {
                    (a, b)
                } else {
                    panic!("bad input")
                }
            });
            let a = it.next().unwrap();
            let b = it.next().unwrap();

            (a.0 >= b.0 && a.1 <= b.1 || b.0 >= a.0 && b.1 <= a.1) as u32
        })
        .sum::<u32>();

    println!("{}", sol);
}
