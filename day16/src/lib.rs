mod parser;
mod simulator;
mod structs;

use simulator::{
    calculate_total_released_pressure, convert_plan_to_paths, create_valve_hash_map, get_cost,
    get_dogs,
};

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
    let size = closed_valves.len();
    let mut all_paths = Vec::new();
    let mut dfs = Vec::new();
    dfs.push(("AA", Vec::new(), Vec::new()));
    loop {
        if dfs.is_empty() {
            break;
        }
        let (from, visited, total_costs) = dfs.pop().unwrap();
        let total_cost: usize = total_costs.iter().sum();
        if visited.len() == size || total_cost >= time {
            let mut plan = visited.clone();
            plan.reverse();
            let paths = convert_plan_to_paths(plan, &dogs);
            all_paths.push(paths);
            continue;
        }
        for to in &closed_valves {
            if visited.contains(to) {
                continue;
            }
            let cost = get_cost(from, to, &dogs);
            let mut new_visited = vec![to.clone()];
            let mut new_total_costs = vec![cost];
            new_visited.append(&mut visited.clone());
            new_total_costs.append(&mut total_costs.clone());
            dfs.push((*to, new_visited, new_total_costs));
        }
    }
    let mut result = 0;
    for paths in all_paths {
        let total_released_pressure =
            calculate_total_released_pressure(paths.clone(), &valves_map, time);
        if total_released_pressure > result {
            result = total_released_pressure;
        }
    }
    Some(result)
}

pub fn process_part2(content: String) -> Option<usize> {
    None
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
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
