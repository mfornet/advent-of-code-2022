fn main() {
    let input = aoc::input_str(file!());

    let sol: i32 = input
        .split("\n\n")
        .map(|chunks| {
            chunks
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", sol);
}
