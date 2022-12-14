mod parser;

use parser::{parse, Element};
use std::cmp::Ordering;

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
        let result = compare(&left, &right);
        println!("{}: {:?}", pair_number, result);
        if result.is_le() {
            sum += pair_number;
        }
    }
    Some(sum)
}

fn compare(left: &Element, right: &Element) -> Ordering {
    match (&left, &right) {
        (Element::Atom(left), Element::Atom(right)) => left.cmp(&right),
        (Element::Atom(_), Element::List(_)) => compare(&left.to_list(), right),
        (Element::List(_), Element::Atom(_)) => compare(left, &right.to_list()),
        (Element::List(left), Element::List(right)) => compare_list(left, right),
    }
}

fn compare_list(left: &Vec<Element>, right: &Vec<Element>) -> Ordering {
    let mut finger: usize = 0;
    loop {
        let left = left.get(finger);
        let right = right.get(finger);
        match (left, right) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(left), Some(right)) => {
                let ordering = compare(left, right);
                if !ordering.is_eq() {
                    return ordering;
                }
            }
        }
        finger += 1;
    }
}

pub fn process_part2(content: String) -> Option<usize> {
    None
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
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
