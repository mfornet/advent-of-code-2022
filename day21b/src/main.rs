use std::collections::HashMap;

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

#[derive(Clone, Debug)]
enum Op {
    Mul,
    Sum,
    Sub,
    Div,
    Eq,
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
    Value(symbolic::Monkey),
}

fn compute(id: String, monkeys: &mut HashMap<String, Monkey>) -> symbolic::Monkey {
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
        Op::Eq => left - right,
    };

    monkeys.insert(id, Monkey::Value(ret));

    ret
}

mod fraction {
    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Debug, Clone, Copy)]
    pub struct Fraction {
        num: i128,
        den: i128,
    }

    impl Fraction {
        pub fn new(num: i128, den: i128) -> Fraction {
            assert_ne!(den, 0);

            let gcd = super::gcd(num, den);
            let mut num = num / gcd;
            let mut den = den / gcd;

            if den < 0 {
                num = -num;
                den = -den;
            }

            Fraction { num, den }
        }

        pub fn is_zero(&self) -> bool {
            self.num == 0
        }
    }

    impl Add for Fraction {
        type Output = Fraction;

        fn add(self, other: Fraction) -> Fraction {
            let num = self.num * other.den + other.num * self.den;
            let den = self.den * other.den;

            Fraction::new(num, den)
        }
    }

    impl Sub for Fraction {
        type Output = Fraction;

        fn sub(self, other: Fraction) -> Fraction {
            let num = self.num * other.den - other.num * self.den;
            let den = self.den * other.den;

            Fraction::new(num, den)
        }
    }

    impl Mul for Fraction {
        type Output = Fraction;

        fn mul(self, other: Fraction) -> Fraction {
            let num = self.num * other.num;
            let den = self.den * other.den;

            Fraction::new(num, den)
        }
    }

    impl Div for Fraction {
        type Output = Fraction;

        fn div(self, other: Fraction) -> Fraction {
            let num = self.num * other.den;
            let den = self.den * other.num;

            Fraction::new(num, den)
        }
    }
}

mod symbolic {
    use super::fraction::*;
    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Clone, Debug, Copy)]
    pub struct Monkey {
        pub x: Fraction,
        pub f: Fraction,
    }

    impl Monkey {
        pub fn new_x() -> Monkey {
            Monkey {
                x: Fraction::new(1, 1),
                f: Fraction::new(0, 1),
            }
        }

        pub fn new_f(v: i128) -> Monkey {
            Monkey {
                x: Fraction::new(0, 1),
                f: Fraction::new(v, 1),
            }
        }
    }

    impl Add for Monkey {
        type Output = Monkey;

        fn add(self, other: Monkey) -> Monkey {
            Monkey {
                x: self.x + other.x,
                f: self.f + other.f,
            }
        }
    }

    impl Mul for Monkey {
        type Output = Monkey;

        fn mul(self, other: Monkey) -> Monkey {
            assert!(self.x.is_zero() || other.x.is_zero());

            Monkey {
                x: self.x * other.f + self.f * other.x,
                f: self.f * other.f,
            }
        }
    }

    impl Sub for Monkey {
        type Output = Monkey;

        fn sub(self, other: Monkey) -> Monkey {
            Monkey {
                x: self.x - other.x,
                f: self.f - other.f,
            }
        }
    }

    impl Div for Monkey {
        type Output = Monkey;

        fn div(self, other: Monkey) -> Monkey {
            assert!(other.x.is_zero());

            Monkey {
                x: self.x / other.f,
                f: self.f / other.f,
            }
        }
    }
}

fn main() {
    let mut monkeys = HashMap::new();

    for line in aoc::input_lines(file!()) {
        let mut tokens = line.split(": ");

        let name = tokens.next().unwrap();
        let expr = tokens.next().unwrap();

        if let Ok(num) = expr.parse::<i128>() {
            let num = if name == "humn" {
                symbolic::Monkey::new_x()
            } else {
                symbolic::Monkey::new_f(num)
            };
            monkeys.insert(name.to_string(), Monkey::Value(num));
        } else {
            let mut tokens = expr.split(" ");

            let left = tokens.next().unwrap();
            let op = tokens.next().unwrap();
            let right = tokens.next().unwrap();

            let op = if name == "root" {
                Op::Eq
            } else {
                match op {
                    "*" => Op::Mul,
                    "+" => Op::Sum,
                    "-" => Op::Sub,
                    "/" => Op::Div,
                    _ => panic!("Unknown op {}", op),
                }
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

    let ret = compute("root".to_string(), &mut monkeys);
    let ret = (fraction::Fraction::new(0, 1) - ret.f) / ret.x;

    println!("{:?}", ret);
}
