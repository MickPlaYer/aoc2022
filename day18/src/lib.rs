mod parser;
mod structs;

use parser::parse_line;
use structs::Point;

pub fn process_part1(content: String) -> Option<usize> {
    let mut points = Vec::new();
    for line in content.lines() {
        let (x, y, z) = parse_line(line);
        let mut new_point = Point::new(x, y, z);
        for point in points.iter_mut() {
            new_point.attch(point);
        }
        points.push(new_point);
    }
    let result = points
        .into_iter()
        .map(|point| point.get_face_count())
        .sum::<usize>();
    Some(result)
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 18;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(64), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(3470), answer);
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
