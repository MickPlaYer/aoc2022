mod parser;
mod simulator;
mod structs;

use itertools::Itertools;
use shared::log;
use simulator::{
    calculate_total_released_pressure, convert_plan_to_paths, create_valve_hash_map, get_cost,
    get_dogs,
};
use std::{collections::HashMap, rc::Rc};
use structs::{Dog, Valve};

pub fn process_part1(content: String) -> Option<usize> {
    let time = 30;
    let valves_map = create_valve_hash_map(content);
    let dogs = get_dogs(&valves_map);
    let closed_valves = valves_map
        .values()
        .filter_map(|valve| {
            if valve.get_flow_rate() == 0 {
                return None;
            }
            Some(valve.get_name())
        })
        .collect::<Vec<_>>();
    for pair in closed_valves.iter().combinations(2) {
        get_cost(pair[0], pair[1], &dogs);
    }
    let size = closed_valves.len();
    let result = idk_maybe_use_the_dfs_to_plan_the_order_then_open_valves(
        size,
        time,
        closed_valves,
        &dogs,
        &valves_map,
    );
    Some(result)
}

fn idk_maybe_use_the_dfs_to_plan_the_order_then_open_valves(
    size: usize,
    time: usize,
    closed_valves: Vec<&str>,
    dogs: &HashMap<&str, Rc<Dog>>,
    valves_map: &HashMap<String, Valve>,
) -> usize {
    let mut result = 0;
    let mut all_paths = Vec::new();
    let mut dfs_stack = Vec::new();
    dfs_stack.push(("AA", Vec::new(), 0));
    loop {
        if dfs_stack.is_empty() {
            break;
        }
        let (from, visited, total_cost) = dfs_stack.pop().unwrap();
        if visited.len() == size || total_cost >= time {
            let mut plan = visited.clone();
            plan.reverse();
            let paths = convert_plan_to_paths(plan, &dogs);
            let total_released_pressure =
                calculate_total_released_pressure(paths.clone(), &valves_map, time);
            if total_released_pressure > result {
                result = total_released_pressure;
            }
            all_paths.push(paths);
            continue;
        }
        for to in &closed_valves {
            if visited.contains(to) {
                continue;
            }
            let cost = get_cost(from, to, &dogs);
            let mut new_visited = vec![to.clone()];
            new_visited.append(&mut visited.clone());
            dfs_stack.push((*to, new_visited, total_cost + cost));
        }
    }
    result
}

pub fn process_part2(content: String) -> Option<usize> {
    let time = 26;
    let valves_map_a = create_valve_hash_map(content.clone());
    let valves_map_b = create_valve_hash_map(content.clone());
    let valves_map_c = create_valve_hash_map(content.clone());
    let mut closed_valves = valves_map_c
        .values()
        .filter_map(|valve| {
            if valve.get_flow_rate() == 0 {
                return None;
            }
            Some(valve.get_name().clone())
        })
        .collect::<Vec<_>>();
    closed_valves.sort();
    let size = closed_valves.len();
    let mut result = 0;
    for split_index in 0..(size / 2 + 1) {
        log(format!("{}", split_index).as_str());
        for my_valves in closed_valves.clone().into_iter().combinations(split_index) {
            let mut elephant_s_valvs = Vec::new();
            for valve in closed_valves.clone() {
                if !my_valves.contains(&valve) {
                    elephant_s_valvs.push(valve)
                }
            }
            let total_released_pressure_by_me =
                idk_maybe_use_the_dfs_to_plan_the_order_then_open_valves(
                    my_valves.clone().len(),
                    time.clone(),
                    my_valves.clone(),
                    &get_dogs(&valves_map_a),
                    &valves_map_a.clone(),
                );
            let total_released_pressure_by_elephant =
                idk_maybe_use_the_dfs_to_plan_the_order_then_open_valves(
                    elephant_s_valvs.clone().len(),
                    time.clone(),
                    elephant_s_valvs.clone(),
                    &get_dogs(&valves_map_b),
                    &valves_map_b.clone(),
                );
            let total_released_pressure =
                total_released_pressure_by_me + total_released_pressure_by_elephant;
            if total_released_pressure > result {
                result = total_released_pressure;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 16;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(1651), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(1923), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(1707), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(2594), answer);
    }
}
