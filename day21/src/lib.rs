mod parser;

use parser::parse;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
enum Number {
    M(isize),
    H(isize, isize, f64),
}

impl Number {
    fn add(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::M(a), Number::M(b)) => Number::M(a + b),
            (Number::M(a), Number::H(m, d, b)) => Number::H(*m, *d, *a as f64 + b),
            (Number::H(m, d, a), Number::M(b)) => Number::H(*m, *d, a + *b as f64),
            (Number::H(_, _, _), Number::H(_, _, _)) => panic!("Two human!"),
        }
    }

    fn sub(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::M(a), Number::M(b)) => Number::M(a - b),
            (Number::M(a), Number::H(m, d, b)) => Number::H(-m, *d, *a as f64 - b),
            (Number::H(m, d, a), Number::M(b)) => Number::H(*m, *d, a - *b as f64),
            (Number::H(_, _, _), Number::H(_, _, _)) => panic!("Two human!"),
        }
    }

    fn mul(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::M(a), Number::M(b)) => Number::M(a * b),
            (Number::M(a), Number::H(m, d, b)) => Number::H(a * m, *d, *a as f64 * b),
            (Number::H(m, d, a), Number::M(b)) => Number::H(m * b, *d, a * *b as f64),
            (Number::H(_, _, _), Number::H(_, _, _)) => panic!("Two human!"),
        }
    }

    fn div(&self, other: &Number) -> Number {
        match (self, other) {
            (Number::M(a), Number::M(b)) => Number::M(a / b),
            (Number::M(_), Number::H(_, _, _)) => panic!("Have to Find the Inverse F(x)!"),
            (Number::H(m, d, a), Number::M(b)) => {
                dbg!(a, *b as f64, a / *b as f64);
                Number::H(*m, d * b, a / *b as f64)
            }
            (Number::H(_, _, _), Number::H(_, _, _)) => panic!("Two human!"),
        }
    }
}

#[derive(Debug)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Match,
}

#[derive(Debug)]
enum MonkeyType {
    Yell(Number),
    Operator(OperationType, String, String),
    TryMatch(Number, Number),
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
            MonkeyType::Yell(Number::M(number)) => Some(number),
            MonkeyType::TryMatch(Number::M(number), Number::H(m, d, i))
            | MonkeyType::TryMatch(Number::H(m, d, i), Number::M(number)) => {
                // (m / d) * human + i = number, solve human
                let number = (number as f64 - i) * d as f64;
                let number = number / m as f64;
                Some(number as isize)
            }
            _ => None,
        }
    }
}

fn ask_from_root(root: &MonkeyRecord, hash_map: &HashMap<String, MonkeyRecord>) -> Option<isize> {
    let mut search = vec![root];
    loop {
        if search.is_empty() {
            break;
        }
        let monkey_record = search.pop().unwrap();
        let binding = monkey_record.get_monkey_type();
        let mut monkey_type = binding.borrow_mut();
        if let MonkeyType::Operator(operation_type, wait_a, wait_b) = &*monkey_type {
            let wait_a = hash_map.get(wait_a).unwrap();
            let wait_b = hash_map.get(wait_b).unwrap();
            if let MonkeyType::Yell(number_a) = &*wait_a.get_monkey_type().borrow() {
                if let MonkeyType::Yell(number_b) = &*wait_b.get_monkey_type().borrow() {
                    let number = match operation_type {
                        OperationType::Add => number_a.add(number_b),
                        OperationType::Sub => number_a.sub(number_b),
                        OperationType::Mul => number_a.mul(number_b),
                        OperationType::Div => number_a.div(number_b),
                        OperationType::Match => {
                            *monkey_type = MonkeyType::TryMatch(number_a.clone(), number_b.clone());
                            break;
                        }
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
    root.get_number()
}

fn fix_mistranslation(hash_map: &mut HashMap<String, MonkeyRecord>) {
    let root = hash_map.get("root").unwrap();
    let mut monkey_type = root.monkey_type.borrow_mut();
    let new_monkey_type = match &*monkey_type {
        MonkeyType::Operator(_, wait_a, wait_b) => {
            MonkeyType::Operator(OperationType::Match, wait_a.clone(), wait_b.clone())
        }
        _ => panic!("Root Monkey Need to Match!"),
    };
    *monkey_type = new_monkey_type;
    let human = hash_map.get("humn").unwrap();
    let mut monkey_type = human.monkey_type.borrow_mut();
    let new_monkey_type = match &*monkey_type {
        MonkeyType::Yell(_) => MonkeyType::Yell(Number::H(1, 1, 0f64)),
        _ => panic!("Human Need to Yell!"),
    };
    *monkey_type = new_monkey_type;
}

pub fn process_part1(content: String) -> Option<isize> {
    let hash_map = parse(&content);
    let root = hash_map.get("root").unwrap();
    ask_from_root(root, &hash_map)
}

pub fn process_part2(content: String) -> Option<isize> {
    let mut hash_map = parse(&content);
    fix_mistranslation(&mut hash_map);
    let root = hash_map.get("root").unwrap();
    ask_from_root(root, &hash_map)
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
        assert_eq!(Some(301), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(3349136384441), answer);
    }
}
