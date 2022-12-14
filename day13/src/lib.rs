mod parser;
mod structs;

use parser::parse;
use structs::Element;

pub fn process_part1(content: String) -> Option<usize> {
    let mut pairs = Vec::new();
    content
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| *line == "")
        .for_each(|e| pairs.push((parse(e[0]).unwrap(), parse(e[1]).unwrap())));
    let mut sum = 0;
    for (index, (left, right)) in pairs.into_iter().enumerate() {
        let pair_number = index + 1;
        let result = left.cmp(&right);
        if result.is_le() {
            sum += pair_number;
        }
    }
    Some(sum)
}

pub fn process_part2(content: String) -> Option<usize> {
    let divider_packets = [parse("[[2]]").unwrap(), parse("[[6]]").unwrap()];
    let mut packets = content
        .lines()
        .filter_map(|line| if line == "" { None } else { parse(line) })
        .collect::<Vec<Element>>();
    packets.extend_from_slice(&divider_packets);
    packets.sort();
    let mut decoder_key = 1;
    for (index, packet) in packets.into_iter().enumerate() {
        let pair_number = index + 1;
        if divider_packets.contains(&packet) {
            decoder_key *= pair_number;
        }
    }
    Some(decoder_key)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 13;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(13), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(5623), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(140), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(20570), answer);
    }
}
