mod parser;
mod structs;

use parser::parse;
use shared::log;
use std::collections::HashSet;
use structs::{Blueprint, State};

fn simulate(blueprint: &Blueprint, time: usize) -> usize {
    let mut first_geode_robot = None;
    let mut second_geode_robot = None;
    let mut third_geode_robot = None;
    let state = State::new();
    let mut futures = vec![state];
    for i in 0..time {
        let mut next_futures = HashSet::new();
        for state in futures.into_iter() {
            let futures = state.try_build_robot(blueprint);
            for mut state in futures.into_iter() {
                state.collect_resource();
                state.finished_robot();
                next_futures.insert(state);
            }
        }
        futures = next_futures.into_iter().collect();
        log(format!("{}: {}", i + 1, futures.len()).as_str());
        if first_geode_robot.is_none() {
            if futures.iter().any(|state| state.geode_robots == 1) {
                log(format!("{}: first geode robot!", i + 1).as_str());
                first_geode_robot = Some(i);
            }
        } else {
            let time = first_geode_robot.unwrap();
            if time + 2 == i {
                log(format!("{}: time to eliminate!(1)", i + 1).as_str());
                futures.retain(|state| state.geode_robots > 0);
            }
        }
        if second_geode_robot.is_none() {
            if futures.iter().any(|state| state.geode_robots == 2) {
                log(format!("{}: second geode robot!", i + 1).as_str());
                second_geode_robot = Some(i);
            }
        } else {
            let time = second_geode_robot.unwrap();
            if time + 3 == i {
                log(format!("{}: time to eliminate!(2)", i + 1).as_str());
                futures.retain(|state| state.geode_robots > 1);
            }
        }
        if third_geode_robot.is_none() {
            if futures.iter().any(|state| state.geode_robots == 3) {
                log(format!("{}: third geode robot!", i + 1).as_str());
                third_geode_robot = Some(i);
            }
        } else {
            let time = third_geode_robot.unwrap();
            if time + 4 == i {
                log(format!("{}: time to eliminate!(3)", i + 1).as_str());
                futures.retain(|state| state.geode_robots > 2);
            }
        }
    }
    futures.into_iter().map(|state| state.geode).max().unwrap()
}

pub fn process_part1(content: String) -> Option<usize> {
    let blueprints = parse(&content);
    let result = blueprints
        .iter()
        .map(|blueprint| {
            let geode = simulate(blueprint, 24);
            dbg!(blueprint.id, geode);
            geode * blueprint.id
        })
        .collect::<Vec<usize>>();
    Some(result.iter().sum())
}

pub fn process_part2(content: String) -> Option<usize> {
    let blueprints = parse(&content);
    let result = blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let geode = simulate(blueprint, 32);
            dbg!(blueprint.id, geode);
            geode
        })
        .collect::<Vec<usize>>();
    Some(result.iter().product())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 19;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(33), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(960), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(3472), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(2040), answer);
    }
}
