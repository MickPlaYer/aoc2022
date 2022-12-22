mod parser;
mod structs;

use parser::{parse_part1, parse_part2};

pub fn process_part1(content: String) -> Option<usize> {
    let mut monkey_map = parse_part1(&content);
    monkey_map.follow_path();
    let row = (monkey_map.current_point.y + 1) as usize;
    let column = (monkey_map.current_point.x + 1) as usize;
    dbg!(row, column, &monkey_map.current_facing);
    Some(1000 * row + 4 * column + monkey_map.current_facing as usize)
}

pub fn process_part2(content: String, face_size: usize) -> Option<usize> {
    let mut human_map = parse_part2(&content, face_size);
    human_map.follow_path();
    human_map.get_result()
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 22;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(6032), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(77318), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content, 4);
        assert_eq!(Some(5031), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content, 50);
        assert_eq!(Some(126017), answer);
    }
}
