fn main() {
    let col1 = "ABC";
    let col2 = "XYZ";

    let sol: i32 = aoc::input_lines(file!())
        .map(|line| {
            let mut tokens = line.split(' ');
            let p1 = col1.find(tokens.next().unwrap()).unwrap() as i32;
            let mut p2 = col2.find(tokens.next().unwrap()).unwrap() as i32;
            p2 = (p2 + p1 - 1 + 3) % 3;
            p2 + 1 + ((p2 - p1 + 4) % 3 * 3)
        })
        .sum();

    println!("{}", sol);
}
