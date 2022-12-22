use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Op {
    Mul,
    Sum,
    Sub,
    Div,
}

#[derive(Clone, Debug)]
struct Binary {
    left: String,
    right: String,
    op: Op,
}

#[derive(Clone, Debug)]
enum Monkey {
    Binary(Binary),
    Value(i64),
}

fn compute(id: String, monkeys: &mut HashMap<String, Monkey>) -> i64 {
    if let Monkey::Value(v) = monkeys[&id] {
        return v;
    }

    let binary = match monkeys[&id] {
        Monkey::Binary(ref binary) => binary.clone(),
        _ => panic!("Expected binary"),
    };

    let left = compute(binary.left, monkeys);
    let right = compute(binary.right, monkeys);

    let ret = match binary.op {
        Op::Mul => left * right,
        Op::Sum => left + right,
        Op::Sub => left - right,
        Op::Div => left / right,
    };

    monkeys.insert(id, Monkey::Value(ret));

    ret
}

fn main() {
    let mut monkeys = HashMap::new();

    for line in aoc::input_lines(file!()) {
        let mut tokens = line.split(": ");

        let name = tokens.next().unwrap();
        let expr = tokens.next().unwrap();

        if let Ok(num) = expr.parse::<i64>() {
            monkeys.insert(name.to_string(), Monkey::Value(num));
        } else {
            let mut tokens = expr.split(' ');

            let left = tokens.next().unwrap();
            let op = tokens.next().unwrap();
            let right = tokens.next().unwrap();

            let op = match op {
                "*" => Op::Mul,
                "+" => Op::Sum,
                "-" => Op::Sub,
                "/" => Op::Div,
                _ => panic!("Unknown op {}", op),
            };

            monkeys.insert(
                name.to_string(),
                Monkey::Binary(Binary {
                    left: left.to_string(),
                    right: right.to_string(),
                    op,
                }),
            );
        }
    }

    // Compute the value yelled by the monkey named "root".
    let ret = compute("root".to_string(), &mut monkeys);
    println!("{}", ret);
}
