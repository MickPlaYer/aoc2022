mod parser;

use parser::parse;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum MonkeyType {
    Yell(isize),
    Operator(OperationType, String, String),
}

#[derive(Debug)]
struct MonkeyRecord {
    name: String,
    monkey_type: Rc<RefCell<MonkeyType>>,
}

impl MonkeyRecord {
    fn new(name: String, monkey_type: MonkeyType) -> Self {
        let monkey_type = Rc::new(RefCell::new(monkey_type));
        Self { name, monkey_type }
    }

    fn get_monkey_type(&self) -> Rc<RefCell<MonkeyType>> {
        self.monkey_type.clone()
    }

    fn get_number(&self) -> Option<isize> {
        match *self.monkey_type.borrow() {
            MonkeyType::Yell(number) => Some(number),
            MonkeyType::Operator(_, _, _) => None,
        }
    }
}

pub fn process_part1(content: String) -> Option<isize> {
    let mut hash_map = HashMap::new();
    let monkey_records = parse(&content);
    for monkey_record in monkey_records {
        hash_map.insert(monkey_record.name.clone(), monkey_record);
    }
    let root = hash_map.get("root").unwrap();
    let mut search = vec![root];
    loop {
        if search.is_empty() {
            break;
        }
        let monkey_record = search.pop().unwrap();
        let binding = monkey_record.get_monkey_type();
        let mut monkey_type = binding.borrow_mut();
        match &*monkey_type {
            MonkeyType::Yell(_) => (),
            MonkeyType::Operator(operation_type, wait_a, wait_b) => {
                let wait_a = hash_map.get(wait_a).unwrap();
                let wait_b = hash_map.get(wait_b).unwrap();
                if let MonkeyType::Yell(number_a) = &*wait_a.get_monkey_type().borrow() {
                    if let MonkeyType::Yell(number_b) = &*wait_b.get_monkey_type().borrow() {
                        let number = match operation_type {
                            OperationType::Add => number_a + number_b,
                            OperationType::Sub => number_a - number_b,
                            OperationType::Mul => number_a * number_b,
                            OperationType::Div => number_a / number_b,
                        };
                        let new_monkey_type = MonkeyType::Yell(number);
                        *monkey_type = new_monkey_type;
                        continue;
                    }
                }
                search.push(monkey_record);
                search.push(wait_a);
                search.push(wait_b);
            }
        }
    }
    let root = hash_map.get("root").unwrap();
    dbg!(root);
    root.get_number()
}

pub fn process_part2(content: String) -> Option<isize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 21;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(152), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(81075092088442), answer);
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
