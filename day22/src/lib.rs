mod parser;
mod structs;

use parser::parse;

pub fn process_part1(content: String) -> Option<usize> {
    let mut monkey_map = parse(&content);
    monkey_map.follow_path();
    let row = (monkey_map.current_point.y + 1) as usize;
    let column = (monkey_map.current_point.x + 1) as usize;
    dbg!(row, column, &monkey_map.current_facing);
    Some(1000 * row + 4 * column + monkey_map.current_facing as usize)
}

pub fn process_part2(content: String) -> Option<usize> {
    None
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
