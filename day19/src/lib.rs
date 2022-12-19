mod parser;
mod structs;

use parser::parse;
use shared::log;
use std::collections::HashSet;
use structs::{Blueprint, State};

fn simulate(blueprint: &Blueprint) -> usize {
    let state = State::new();
    let mut futures = vec![state];
    for i in 0..24 {
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
    }
    futures.into_iter().map(|state| state.geode).max().unwrap()
}

pub fn process_part1(content: String) -> Option<usize> {
    let blueprints = parse(&content);
    let result = blueprints
        .iter()
        .map(|blueprint| {
            let geode = simulate(blueprint);
            dbg!(blueprint.id, geode);
            geode * blueprint.id
        })
        .collect::<Vec<usize>>();
    dbg!(&result);
    Some(result.iter().sum())
}

pub fn process_part2(content: String) -> Option<usize> {
    None
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
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
