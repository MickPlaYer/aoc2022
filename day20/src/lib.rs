use std::{collections::VecDeque, fmt::Debug};

#[derive(Clone)]
struct Cell {
    id: usize,
    number: isize,
    checked: bool,
}

impl Cell {
    fn new(id: usize, number: isize) -> Self {
        Self {
            id,
            number,
            checked: false,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cell id: {} number: {}, checked: {}",
            &self.id, &self.number, &self.checked
        )
    }
}

fn get_coordinate_sum(list: Vec<Cell>) -> isize {
    let mut coordinate_sum = 0;
    let loop_length = list.len();
    let zero_poition = list.iter().position(|cell| cell.number == 0).unwrap();
    for poition in [1000, 2000, 3000] {
        let poition = (poition + zero_poition) % loop_length;
        let cell = &list[poition];
        dbg!(cell.number);
        coordinate_sum += cell.number;
    }
    coordinate_sum
}

fn parse(content: &str) -> VecDeque<Cell> {
    content
        .lines()
        .enumerate()
        .map(|(i, e)| Cell::new(i + 1, e.parse().unwrap()))
        .collect()
}

pub fn process_part1(content: String) -> Option<isize> {
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
        // with loop_length 6: 2, 8, 14, -4, -10, -16 are the same, and step will be 2
        let step = cell.number.checked_rem_euclid(loop_length).unwrap() as usize;
        list.insert(step, cell);
    }
    Some(get_coordinate_sum(list.into()))
}

pub fn process_part2(content: String) -> Option<isize> {
    let decryption_key = 811589153;
    let mut list: Vec<Cell> = parse(&content).into();
    list.iter_mut()
        .for_each(|cell| cell.number *= decryption_key);
    let ordered_cells = list.clone();
    let loop_length = (list.len() as isize) - 1;
    for _ in 0..10 {
        for ordered_cell in &ordered_cells {
            let position = list
                .iter()
                .position(|cell| cell.id == ordered_cell.id)
                .unwrap();
            let cell = list.remove(position);
            let to_position = (cell.number + position as isize)
                .checked_rem_euclid(loop_length)
                .unwrap() as usize;
            list.insert(to_position, cell);
        }
    }
    Some(get_coordinate_sum(list))
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
        assert_eq!(Some(1623178306), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(7865110481723), answer);
    }
}
