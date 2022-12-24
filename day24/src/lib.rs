mod path_finder;
mod wind_maps;

use path_finder::{find_the_shortest_path, State};
use wind_maps::{get_outer_size, parse_wind_maps, produce_all_wind_status};

pub fn process_part1(content: String) -> Option<usize> {
    let wind_maps = parse_wind_maps(content);
    let all_wind_status = produce_all_wind_status(wind_maps);
    let size_info = get_outer_size(&all_wind_status);
    let start_point = (1, 0);
    let mut start_state = State::new(0, start_point);
    start_state.set_distance(0);
    let end_point = (size_info.0 - 2, size_info.1 - 1);
    let end_state = find_the_shortest_path(&all_wind_status, start_state, end_point);
    end_state.and_then(|state| Some(state.get_distance()))
}

pub fn process_part2(content: String) -> Option<usize> {
    let wind_maps = parse_wind_maps(content);
    let all_wind_status = produce_all_wind_status(wind_maps);
    let size_info = get_outer_size(&all_wind_status);
    let start_point = (1, 0);
    let mut start_state = State::new(0, start_point);
    start_state.set_distance(0);
    let end_point = (size_info.0 - 2, size_info.1 - 1);
    let end_state = find_the_shortest_path(&all_wind_status, start_state, end_point).unwrap();
    let end_state = find_the_shortest_path(&all_wind_status, end_state, start_point).unwrap();
    let end_state = find_the_shortest_path(&all_wind_status, end_state, end_point).unwrap();
    Some(end_state.get_distance())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 24;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(18), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(242), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(54), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(720), answer);
    }
}
