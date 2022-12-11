mod structs;

use std::{cell::RefCell, collections::HashMap, rc::Rc};
use structs::{parse_monkeys, Monkey};

type RMonkey = Rc<RefCell<Monkey>>;
type RMonkeys = Vec<RMonkey>;

fn simulate_rounds(monkeys: Vec<Monkey>, rounds: usize, very_worried: bool) -> RMonkeys {
    let common_multiple_of_divisible_by = monkeys
        .iter()
        .fold(1, |result, monkey| result * monkey.get_test_divisible_by());
    let (mut monkeys, mut monkey_hash_map) = create_hashmap_with_refcell(monkeys);
    for _ in 0..rounds {
        for monkey in monkeys.iter_mut() {
            loop {
                let inspection = monkey.borrow_mut().inspect_item(very_worried);
                if inspection.is_none() {
                    break;
                }
                let (mut item, monkey_id) = inspection.unwrap();
                while item > common_multiple_of_divisible_by {
                    item = item % common_multiple_of_divisible_by
                }
                monkey_hash_map
                    .get_mut(&monkey_id)
                    .unwrap()
                    .borrow_mut()
                    .push_item(item);
            }
        }
    }
    print_monkey_inspected_times(&monkeys);
    monkeys
}

fn create_hashmap_with_refcell(monkeys: Vec<Monkey>) -> (RMonkeys, HashMap<usize, RMonkey>) {
    let mut new_monkeys = Vec::new();
    let mut monkey_hash_map = HashMap::new();
    monkeys.into_iter().for_each(|monkey| {
        let id = monkey.id;
        let monkey = Rc::new(RefCell::new(monkey));
        new_monkeys.push(monkey.clone());
        monkey_hash_map.insert(id, monkey);
    });
    (new_monkeys, monkey_hash_map)
}

fn print_monkey_inspected_times(monkeys: &RMonkeys) {
    for monkey in monkeys.iter() {
        let monkey = monkey.borrow();
        println!(
            "Monkey {} inspected items {} times.",
            monkey.id, monkey.inspected_times
        );
    }
}

fn calculate_result_by_most_two_inspected_times_monkeys(mut monkeys: RMonkeys) -> usize {
    monkeys.sort_by(|a, b| b.borrow().inspected_times.cmp(&a.borrow().inspected_times));
    let result = monkeys
        .iter()
        .take(2)
        .fold(1, |result, monkey| result * monkey.borrow().inspected_times);
    result
}

pub fn process_part1(content: String) -> Option<usize> {
    let monkeys = parse_monkeys(&content).unwrap();
    let monkeys = simulate_rounds(monkeys, 20, false);
    let result = calculate_result_by_most_two_inspected_times_monkeys(monkeys);
    Some(result)
}

pub fn process_part2(content: String) -> Option<usize> {
    let monkeys = parse_monkeys(&content).unwrap();
    let monkeys = simulate_rounds(monkeys, 10000, true);
    let result = calculate_result_by_most_two_inspected_times_monkeys(monkeys);
    Some(result)
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
        assert_eq!(Some(2713310158), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(15117269860), answer);
    }
}
