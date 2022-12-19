use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

#[derive(Clone)]
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

fn go(rem: u64, state: State, blueprint: &Blueprint) -> u64 {
    if rem == 0 {
        state.geode
    } else {
        let mut best = 0;

        // Build geode robot
        let can_build_geode = blueprint.can_build_geode(&state);
        if can_build_geode >= 1 {
            let mut n_state = state.clone();
            n_state.update();
            n_state.ore -= blueprint.geode_ore;
            n_state.obsidian -= blueprint.geode_obsidian;
            n_state.robot_geode += 1;
            let cur = go(rem - 1, n_state, blueprint);
            best = std::cmp::max(best, cur);
        }

        // Build obsidian robot
        let can_build_obsidian = blueprint.can_build_obsidian(&state);
        if can_build_obsidian >= 1 {
            let mut n_state = state.clone();
            n_state.update();
            n_state.ore -= blueprint.obsidian_ore;
            n_state.clay -= blueprint.obsidian_clay;
            n_state.robot_obsidian += 1;
            let cur = go(rem - 1, n_state, blueprint);
            best = std::cmp::max(best, cur);
        }

        // Build clay robot
        let can_build_clay = blueprint.can_build_clay(&state);
        if can_build_geode < 2 && can_build_clay >= 1 {
            let mut n_state = state.clone();
            n_state.update();
            n_state.ore -= blueprint.clay_ore;
            n_state.robot_clay += 1;
            let cur = go(rem - 1, n_state, blueprint);
            best = std::cmp::max(best, cur);
        }

        // Build ore robot
        let can_build_ore = blueprint.can_build_ore(&state);
        if can_build_geode < 2 && can_build_ore >= 1 {
            let mut n_state = state.clone();
            n_state.update();
            n_state.ore -= blueprint.ore_ore;
            n_state.robot_ore += 1;
            let cur = go(rem - 1, n_state, blueprint);
            best = std::cmp::max(best, cur);
        }

        // Do nothing
        if can_build_geode < 2 && can_build_obsidian < 2 {
            let mut n_state = state.clone();
            n_state.update();
            let cur = go(rem - 1, n_state, blueprint);
            best = std::cmp::max(best, cur);
        }

        best
    }
}

fn main() {
    let pat = Regex::new(
        r"^Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$",
    );

    let data = aoc::input_lines(file!()).enumerate().collect_vec();

    let answer: u64 = data
        .par_iter()
        .map(|(ix, line)| {
            let caps = pat.as_ref().unwrap().captures(&line).unwrap();
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

            let state = State::new();

            let best = go(24, state, &blueprint);
            println!("{} {}", ix + 1, best);

            (*ix as u64 + 1) * best
        })
        .sum();

    println!("{}", answer);
}
