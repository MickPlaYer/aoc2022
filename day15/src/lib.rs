mod parser;
mod structs;

use parser::parse;
use std::ops::RangeInclusive;
use structs::{Boundary, Point, Record};

fn get_ranges(records: &Vec<Record>, check_line: isize) -> Vec<RangeInclusive<isize>> {
    let mut ranges = Vec::new();
    for record in records {
        if !record.is_cover_line(&check_line) {
            continue;
        }
        ranges.push(record.covered_range_at_line(&check_line));
    }
    ranges
}

fn get_boundary(
    records: &Vec<Record>,
    ranges: Vec<RangeInclusive<isize>>,
    check_line: isize,
) -> Boundary {
    let left = *ranges.iter().map(|range| range.start()).min().unwrap();
    let right = *ranges.iter().map(|range| range.end()).max().unwrap();
    let mut boundary = Boundary::new(left, right);
    for record in records {
        let beacon = record.get_beacon();
        if beacon.y == check_line {
            boundary.set_beacon(beacon.x);
        }
        let sensor = record.get_sensor();
        if sensor.y == check_line {
            boundary.set_sensor(sensor.x);
        }
    }
    for range in ranges {
        for position in range {
            boundary.set_coverd(position);
        }
    }
    boundary
}

fn remove_range(total_ranges: &mut Vec<RangeInclusive<isize>>, range: &RangeInclusive<isize>) {
    let mut new_total_ranges = Vec::new();
    for total_range in total_ranges.iter() {
        let contain_start = total_range.contains(range.start());
        let contain_end = total_range.contains(range.end());
        if !contain_start && !contain_end {
            if !range.contains(total_range.start()) {
                new_total_ranges.push(total_range.clone());
            }
            continue;
        }
        if contain_start {
            let a = *total_range.start()..=(*range.start() - 1);
            if a.clone().count() > 0 {
                new_total_ranges.push(a);
            }
        }
        if contain_end {
            let b = (*range.end() + 1)..=*total_range.end();
            if b.clone().count() > 0 {
                new_total_ranges.push(b);
            }
        }
    }
    total_ranges.clone_from(&new_total_ranges);
}

pub fn process_part1(content: String, check_line: isize) -> Option<usize> {
    let records = parse(&content);
    let ranges = get_ranges(&records, check_line);
    let boundary = get_boundary(&records, ranges, check_line);
    Some(boundary.count_coverd())
}

pub fn process_part2(content: String, range: RangeInclusive<isize>) -> Option<usize> {
    let records = parse(&content);
    let mut point = None;
    for y in range.clone() {
        if point.is_some() {
            break;
        }
        let ranges = get_ranges(&records, y);
        let mut total_ranges = vec![range.clone()];
        for range in &ranges {
            remove_range(&mut total_ranges, range);
        }
        if total_ranges.len() > 0 {
            point = Some(Point::new(*total_ranges[0].start(), y));
            break;
        }
    }
    let point = point?;
    Some((point.x * 4000000 + point.y) as usize)
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
        let answer = process_part2(content, 0..=20);
        assert_eq!(Some(56000011), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content, 0..=4000000);
        assert_eq!(Some(11583882601918), answer);
    }
}
