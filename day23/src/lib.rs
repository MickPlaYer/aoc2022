mod crater_map;
mod simulator;
mod structs;

use crater_map::{count_empty_grounds, draw_map, parse_map};
use simulator::simluate_round;
use structs::Direction;

pub fn process_part1(content: String) -> Option<usize> {
    let mut map = parse_map(content);
    draw_map(&map);
    let mut directions = Direction::default_directions();
    for _ in 0..10 {
        map = simluate_round(&map, &directions).unwrap();
        directions.rotate_left(1);
    }
    Some(count_empty_grounds(&map))
}

pub fn process_part2(content: String) -> Option<usize> {
    let mut map = parse_map(content);
    let mut directions = Direction::default_directions();
    for round in 0.. {
        let result = simluate_round(&map, &directions);
        if result.is_err() {
            return Some(round + 1);
        }
        map = result.unwrap();
        directions.rotate_left(1);
    }
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 23;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(110), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(4288), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(20), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(940), answer);
    }
}
