use std::{collections::VecDeque, fmt::Debug};

struct Cell {
    number: isize,
    checked: bool,
}

impl Cell {
    fn new(number: isize) -> Self {
        Self {
            number,
            checked: false,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cell number: {}, checked: {}",
            &self.number, &self.checked
        )
    }
}

fn parse(content: &str) -> VecDeque<Cell> {
    content
        .lines()
        .map(|e| Cell::new(e.parse().unwrap()))
        .collect()
}

pub fn process_part1(content: String) -> Option<usize> {
    let mut checked_count = 0;
    let mut list = parse(&content);
    let loop_length = (list.len() as isize) - 1;
    loop {
        if checked_count > loop_length {
            break;
        }
        let mut cell = list.pop_front().unwrap();
        if cell.checked {
            list.push_back(cell);
            checked_count += 1;
            continue;
        }
        cell.checked = true;
        // with loop_length 6: 2, 8, 14 or -4, -10, -16 are the same
        let step = cell.number.checked_rem_euclid(loop_length).unwrap() as usize;
        println!("{} <= {:?}", step, &cell);
        list.insert(step, cell);
    }
    dbg!(&list);
    let mut coordinate_sum = 0;
    let loop_length = list.len();
    let zero_poition = list.iter().position(|e| e.number == 0).unwrap();
    for poition in [1000, 2000, 3000] {
        let poition = (poition + zero_poition) % loop_length;
        let cell = &list[poition];
        dbg!(cell);
        coordinate_sum += cell.number;
    }
    Some(coordinate_sum as usize)
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 20;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(3), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(8372), answer);
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
