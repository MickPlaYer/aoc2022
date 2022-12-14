mod parser;
mod structs;

use parser::parse;
use structs::BigMap;

fn simulate_sand_drop(mut big_map: BigMap) -> usize {
    let mut sand_count = 0;
    loop {
        let is_stoped = big_map.drop_sand();
        if is_stoped {
            break;
        };
        sand_count += 1;
    }
    sand_count
}

pub fn process_part1(content: String) -> Option<usize> {
    let rocks = content.lines().map(parse).collect();
    let big_map = BigMap::new(rocks);
    let sand_count = simulate_sand_drop(big_map);
    Some(sand_count)
}

pub fn process_part2(content: String) -> Option<usize> {
    let rocks = content.lines().map(parse).collect();
    let big_map = BigMap::new_with_floor(rocks);
    let sand_count = simulate_sand_drop(big_map);
    Some(sand_count)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 14;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(24), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(1061), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(93), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(25055), answer);
    }
}
