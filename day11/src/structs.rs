mod monkey_parser;

pub use self::monkey_parser::parse_monkeys;

pub struct Monkey {
    pub id: usize,
    pub inspected_times: usize,
    items: Vec<usize>,
    operation: Operation,
    test: MonkeyTest,
}

impl Monkey {
    fn new(id: usize, items: Vec<usize>, operation: Operation, test: MonkeyTest) -> Self {
        Self {
            id,
            inspected_times: 0,
            items,
            operation,
            test,
        }
    }

    pub fn inspect_item(&mut self) -> Option<(usize, usize)> {
        let item = self.items.pop()?;
        let item = self.operation.execute(item);
        let item = item / 3;
        let monkey_id = self.test.execute(item);
        self.inspected_times += 1;
        Some((item, monkey_id))
    }

    pub fn push_item(&mut self, item: usize) {
        self.items.push(item);
    }
}

struct MonkeyTest {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl MonkeyTest {
    fn new(divisible_by: usize, if_true: usize, if_false: usize) -> Self {
        Self {
            divisible_by,
            if_true,
            if_false,
        }
    }

    fn execute(&self, item: usize) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

enum Value {
    Old,
    Number(usize),
}
impl Value {
    fn interpret(&self, old_value: usize) -> usize {
        match self {
            Value::Old => old_value,
            Value::Number(value) => *value,
        }
    }
}

enum Operation {
    Multiple(Value, Value),
    Add(Value, Value),
}

impl Operation {
    fn execute(&self, old_value: usize) -> usize {
        match self {
            Operation::Multiple(value_a, value_b) => {
                value_a.interpret(old_value) * value_b.interpret(old_value)
            }
            Operation::Add(value_a, value_b) => {
                value_a.interpret(old_value) + value_b.interpret(old_value)
            }
        }
    }
}
