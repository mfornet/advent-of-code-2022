use itertools::Itertools;

fn main() {
    let pat = regex::Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut stacks = vec![];
    for line in aoc::input_lines(file!()) {
        if line.contains('[') {
            let chars = line.chars().collect_vec();
            let mut i = 0;

            while 4 * i + 1 < chars.len() {
                let c = chars[4 * i + 1];
                if c != ' ' {
                    while stacks.len() <= i {
                        stacks.push(vec![]);
                    }
                    stacks[i].push(c);
                }
                i += 1;
            }
        } else if line.is_empty() {
            for stack in &mut stacks {
                stack.reverse();
            }
        } else if line.starts_with("move") {
            let cap = pat.captures(&line).unwrap();
            let from = cap[2].parse::<usize>().unwrap() - 1;
            let to = cap[3].parse::<usize>().unwrap() - 1;
            let n = cap[1].parse::<usize>().unwrap();

            let from_len = stacks[from].len();
            let values = stacks[from].drain(from_len - n..).collect_vec();
            stacks[to].extend(values);
        }
    }

    // Print the character at the top of each stack
    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
