use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Snafu(String);

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i128> for Snafu {
    fn from(mut n: i128) -> Self {
        let mut digits = vec![];
        while n > 0 {
            digits.push(n % 5);
            n /= 5;
        }

        digits.push(0);

        let mut i = digits.len() - 1;

        loop {
            if digits[i] >= 3 {
                digits[i + 1] += 1;
                digits[i] -= 5;
                i += 1;
            } else if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        while digits.last() == Some(&0) {
            digits.pop();
        }

        let repr: String = digits
            .into_iter()
            .rev()
            .map(|d| match d {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("bad digit"),
            })
            .collect();

        Snafu(repr)
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu(s.to_string()))
    }
}

impl From<Snafu> for i128 {
    fn from(snafu: Snafu) -> Self {
        let mut result = 0;

        for d in snafu.0.chars() {
            result = result * 5
                + match d {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("bad digit"),
                }
        }

        result
    }
}

fn main() {
    let res: Snafu = aoc::input_lines(file!())
        .map(|line| -> i128 { line.parse::<Snafu>().unwrap().into() })
        .sum::<i128>()
        .into();

    println!("{}", res);
}
