use crate::{
    parser::parse,
    structs::{Dog, Path, Valve},
};
use lazy_static::lazy_static;
use std::{collections::HashMap, rc::Rc, sync::Mutex};

pub fn create_valve_hash_map(content: String) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    content.lines().for_each(|line| {
        let valve = parse(line);
        let name = valve.get_name().to_string();
        valves.insert(name, valve);
    });
    valves
}

pub fn convert_plan_to_paths(plan: Vec<&str>, dogs: &HashMap<&str, Rc<Dog>>) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut from = "AA";
    for to in plan {
        let cost = get_cost(from, to, dogs);
        paths.push(Path::new(String::from(to), cost));
        from = to;
    }
    paths
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<(String, String), usize>> = Mutex::new(HashMap::new());
}

pub fn get_cost(from_name: &str, to_name: &str, dogs: &HashMap<&str, Rc<Dog>>) -> usize {
    let mut cache = HASHMAP.lock().unwrap();
    let mut cache_key = vec![from_name.to_string(), to_name.to_string()];
    cache_key.sort();
    let cache_key = (cache_key.pop().unwrap(), cache_key.pop().unwrap());
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    };
    dogs.values().for_each(|dog| dog.reset());
    let from = dogs.get(from_name).unwrap().clone();
    let to = dogs.get(to_name).unwrap().clone();
    let mut history = Vec::new();
    from.set_distance(0);
    history.push(from);
    loop {
        history.sort_by(|a, b| b.get_distance().cmp(&a.get_distance()));
        let current = history.pop();
        if current.is_none() {
            return usize::MAX;
        }
        let current = current.unwrap();
        let neighbors = current.get_friends();
        for neighbor in neighbors {
            if neighbor.is_visited() {
                continue;
            }
            let new_distance = current.get_distance() + 1;
            if new_distance < neighbor.get_distance() {
                neighbor.set_distance(new_distance);
                history.push(neighbor);
            }
        }
        current.set_visited();
        if to.is_visited() {
            break;
        }
    }
    let distance = to.get_distance();
    cache.insert(cache_key, distance);
    distance
}

pub fn get_dogs(valves_map: &HashMap<String, Valve>) -> HashMap<&str, Rc<Dog>> {
    let mut dogs = HashMap::new();
    valves_map.values().for_each(|v| {
        dogs.insert(v.get_name().clone(), Dog::new());
    });
    dogs.clone().keys().for_each(|k| {
        let dog = dogs.get(k.clone()).unwrap();
        let s = valves_map.get(k.clone()).unwrap();
        s.get_tunnels().iter().for_each(|n| {
            let friend = dogs.get(n.as_str().clone()).unwrap();
            dog.push(friend.clone());
        });
    });
    dogs
}

pub fn calculate_total_released_pressure(
    paths: Vec<Path>,
    valves_map: &HashMap<String, Valve>,
    time: usize,
) -> usize {
    let mut time_remains_in_miuntes = time;
    let mut total_released_pressure = 0;
    for path in paths {
        let total_cost = path.get_cost() + 1;
        if time_remains_in_miuntes >= total_cost {
            time_remains_in_miuntes -= total_cost;
            let valve = valves_map.get(path.get_tunnel()).unwrap();
            total_released_pressure += time_remains_in_miuntes * valve.get_flow_rate();
        } else {
            break;
        }
    }
    total_released_pressure
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 16;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn test_get_cost() {
        let content = read_sample(DAY_NUMBER);
        let valves_map = create_valve_hash_map(content);
        let dogs = get_dogs(&valves_map);
        assert_eq!(2, get_cost("EE", "AA", &dogs));
        assert_eq!(2, get_cost("AA", "EE", &dogs));
    }

    #[test]
    fn test_get_cost_2() {
        let content = read_input(DAY_NUMBER);
        let valves_map = create_valve_hash_map(content);
        let dogs = get_dogs(&valves_map);
        assert_eq!(5, get_cost("AA", "PH", &dogs));
        assert_eq!(5, get_cost("PH", "AA", &dogs));
    }

    #[test]
    fn test_calculate_total_released_pressure() {
        let content = read_sample(DAY_NUMBER);
        let valves_map = create_valve_hash_map(content);
        let plan = vec![
            Path::new(String::from("DD"), 1),
            Path::new(String::from("BB"), 2),
            Path::new(String::from("JJ"), 3),
            Path::new(String::from("HH"), 7),
            Path::new(String::from("EE"), 3),
            Path::new(String::from("CC"), 2),
        ];
        let result = calculate_total_released_pressure(plan, &valves_map, 30);
        assert_eq!(1651, result)
    }
}
