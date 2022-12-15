mod parser;
mod structs;

use parser::parse;
use structs::Boundary;

pub fn process_part1(content: String, check_line: isize) -> Option<usize> {
    let records = parse(&content);
    let mut ranges = Vec::new();
    for record in &records {
        if !record.is_cover_line(&check_line) {
            continue;
        }
        ranges.push(record.covered_range_at_line(&check_line));
    }
    let left = *ranges.iter().map(|range| range.start()).min().unwrap();
    let right = *ranges.iter().map(|range| range.end()).max().unwrap();
    let mut boundary = Boundary::new(left, right);
    for record in records {
        let beacon = record.get_beacon();
        if beacon.y == check_line {
            boundary.set_beacon(beacon.x);
        }
    }
    for range in ranges {
        for position in range {
            boundary.set_coverd(position);
        }
    }
    Some(boundary.count_coverd())
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 15;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content, 10);
        assert_eq!(Some(26), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content, 2000000);
        assert_eq!(Some(4985193), answer);
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
