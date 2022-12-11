mod structs;
use crate::structs::parse_monkeys;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn process_part1(content: String) -> Option<usize> {
    let mut monkeys = Vec::new();
    let mut monkey_hash_map = HashMap::new();
    let binding = parse_monkeys(&content).unwrap();
    binding.into_iter().for_each(|monkey| {
        let id = monkey.id;
        let monkey = Rc::new(RefCell::new(monkey));
        monkeys.push(monkey.clone());
        monkey_hash_map.insert(id, monkey);
    });
    for _ in 0..20 {
        for monkey in monkeys.iter_mut() {
            loop {
                let inspection = monkey.borrow_mut().inspect_item();
                if inspection.is_none() {
                    break;
                }
                let (item, monkey_id) = inspection.unwrap();
                monkey_hash_map
                    .get_mut(&monkey_id)
                    .unwrap()
                    .borrow_mut()
                    .push_item(item);
            }
        }
    }
    for monkey in monkeys.iter() {
        let monkey = monkey.borrow();
        println!(
            "Monkey {} inspected items {} times.",
            monkey.id, monkey.inspected_times
        );
    }
    monkeys.sort_by(|a, b| b.borrow().inspected_times.cmp(&a.borrow().inspected_times));
    let result = monkeys
        .iter()
        .take(2)
        .fold(1, |result, monkey| result * monkey.borrow().inspected_times);
    Some(result)
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 11;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(10605), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(55944), answer);
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
