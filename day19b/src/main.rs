use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

mod beam_search;

#[derive(Clone, Debug)]
struct State {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,

    robot_ore: u64,
    robot_clay: u64,
    robot_obsidian: u64,
    robot_geode: u64,
}

impl State {
    fn update(&mut self) {
        self.ore += self.robot_ore;
        self.clay += self.robot_clay;
        self.obsidian += self.robot_obsidian;
        self.geode += self.robot_geode;
    }

    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            robot_ore: 1,
            robot_clay: 0,
            robot_obsidian: 0,
            robot_geode: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_ore: u64,
    clay_ore: u64,
    obsidian_ore: u64,
    obsidian_clay: u64,
    geode_ore: u64,
    geode_obsidian: u64,
}

impl Blueprint {
    fn can_build_geode(&self, state: &State) -> u64 {
        std::cmp::min(
            state.ore / self.geode_ore,
            state.obsidian / self.geode_obsidian,
        )
    }

    fn can_build_obsidian(&self, state: &State) -> u64 {
        std::cmp::min(
            state.ore / self.obsidian_ore,
            state.clay / self.obsidian_clay,
        )
    }

    fn can_build_clay(&self, state: &State) -> u64 {
        state.ore / self.clay_ore
    }

    fn can_build_ore(&self, state: &State) -> u64 {
        state.ore / self.ore_ore
    }
}

#[derive(Debug)]
struct Node<'a> {
    rem: u64,
    state: State,
    blueprint: &'a Blueprint,
}

impl<'a> Node<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            rem: 32,
            state: State::new(),
            blueprint,
        }
    }
}

impl<'a> beam_search::Node for Node<'a> {
    fn children(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut children = Vec::new();

        if self.rem == 0 {
            return children;
        }

        // Build geode robot
        let can_build_geode = self.blueprint.can_build_geode(&self.state);
        if can_build_geode > 0 {
            let mut state = self.state.clone();
            state.update();
            state.ore -= self.blueprint.geode_ore;
            state.obsidian -= self.blueprint.geode_obsidian;
            state.robot_geode += 1;
            children.push(Node {
                rem: self.rem - 1,
                state,
                blueprint: self.blueprint,
            });
        }

        // Build obsidian robot
        let can_build_obsidian = self.blueprint.can_build_obsidian(&self.state);
        if can_build_obsidian > 0 {
            let mut state = self.state.clone();
            state.update();
            state.ore -= self.blueprint.obsidian_ore;
            state.clay -= self.blueprint.obsidian_clay;
            state.robot_obsidian += 1;
            children.push(Node {
                rem: self.rem - 1,
                state,
                blueprint: self.blueprint,
            });
        }

        // Build clay robot
        let can_build_clay = self.blueprint.can_build_clay(&self.state);
        if can_build_clay > 0 {
            let mut state = self.state.clone();
            state.update();
            state.ore -= self.blueprint.clay_ore;
            state.robot_clay += 1;
            children.push(Node {
                rem: self.rem - 1,
                state,
                blueprint: self.blueprint,
            });
        }

        // Build ore robot
        let can_build_ore = self.blueprint.can_build_ore(&self.state);
        if can_build_ore > 0 {
            let mut state = self.state.clone();
            state.update();
            state.ore -= self.blueprint.ore_ore;
            state.robot_ore += 1;
            children.push(Node {
                rem: self.rem - 1,
                state,
                blueprint: self.blueprint,
            });
        }

        // Do nothing
        let mut state = self.state.clone();
        state.update();
        children.push(Node {
            rem: self.rem - 1,
            state,
            blueprint: self.blueprint,
        });

        children
    }

    fn score(&self) -> f64 {
        let mut state = self.state.clone();

        for _ in 0..self.rem {
            // Build geode robot
            let can_build_geode = self.blueprint.can_build_geode(&state);
            if can_build_geode > 0 {
                state.update();
                state.ore -= self.blueprint.geode_ore;
                state.obsidian -= self.blueprint.geode_obsidian;
                state.robot_geode += 1;
                continue;
            }

            // Build obsidian robot
            let can_build_obsidian = self.blueprint.can_build_obsidian(&state);
            if can_build_obsidian > 0 {
                state.update();
                state.ore -= self.blueprint.obsidian_ore;
                state.clay -= self.blueprint.obsidian_clay;
                state.robot_obsidian += 1;
                continue;
            }

            // Build clay robot
            let can_build_clay = self.blueprint.can_build_clay(&state);
            if can_build_clay > 0 {
                state.update();
                state.ore -= self.blueprint.clay_ore;
                state.robot_clay += 1;
                continue;
            }

            // Build ore robot
            let can_build_ore = self.blueprint.can_build_ore(&state);
            if can_build_ore > 0 {
                state.update();
                let mut state = self.state.clone();
                state.ore -= self.blueprint.ore_ore;
                state.robot_ore += 1;
                continue;
            }

            state.update();
        }

        state.geode as f64
    }

    fn real_score(&self) -> u64 {
        self.state.geode
    }
}

fn main() {
    let pat = Regex::new(
        r"^Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$",
    );

    const N: usize = 3;

    let data = aoc::input_lines(file!()).enumerate().take(N).collect_vec();

    let answer = data
        .par_iter()
        .map(|(_, line)| {
            let caps = pat.as_ref().unwrap().captures(line).unwrap();
            let ore_ore = caps[1].parse::<u64>().unwrap();
            let clay_ore = caps[2].parse::<u64>().unwrap();
            let obsidian_ore = caps[3].parse::<u64>().unwrap();
            let obsidian_clay = caps[4].parse::<u64>().unwrap();
            let geode_ore = caps[5].parse::<u64>().unwrap();
            let geode_obsidian = caps[6].parse::<u64>().unwrap();

            let blueprint = Blueprint {
                ore_ore,
                clay_ore,
                obsidian_ore,
                obsidian_clay,
                geode_ore,
                geode_obsidian,
            };

            let source = Node::new(&blueprint);

            beam_search::beam_search(source, 10_000)
        })
        .reduce(|| 1, std::ops::Mul::mul);

    println!("{}", answer);
}
