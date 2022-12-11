use super::{Monkey, MonkeyTest, Operation, Value};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, one_of, space0},
    error::Error,
    multi::{many_m_n, separated_list0},
    sequence::{delimited, preceded, tuple},
    Err, IResult,
};

fn monkey_test_divisible_by_n(input: &str) -> IResult<&str, usize> {
    let (input, (_, divisible_by)) = tuple((
        tuple((tag("Test:"), space0, tag("divisible by"), space0)),
        digit1,
    ))(input)?;
    Ok((input, divisible_by.parse().unwrap()))
}

fn throw_to_monkey_n(input: &str) -> IResult<&str, usize> {
    let (input, (_, monkey_number)) =
        tuple((tuple((tag("throw to monkey"), space0)), digit1))(input)?;
    Ok((input, monkey_number.parse().unwrap()))
}

fn parse_monkey_test(input: &str) -> IResult<&str, MonkeyTest> {
    let if_true = preceded(tuple((tag("If true:"), space0)), throw_to_monkey_n);
    let if_false = preceded(tuple((tag("If false:"), space0)), throw_to_monkey_n);
    let (input, (divisible_by, if_true, if_false)) = tuple((
        delimited(space0, monkey_test_divisible_by_n, newline),
        delimited(space0, if_true, newline),
        delimited(space0, if_false, newline),
    ))(input)?;
    Ok((input, MonkeyTest::new(divisible_by, if_true, if_false)))
}

fn value_or_number(input: &str) -> IResult<&str, &str> {
    Ok(alt((tag("old"), digit1))(input)?)
}

fn parse_value(value_a: &str) -> Value {
    if value_a == "old" {
        return Value::Old;
    }
    Value::Number(value_a.parse().unwrap())
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, (value_a, operation, value_b)) = tuple((
        value_or_number,
        delimited(space0, one_of("+*"), space0),
        value_or_number,
    ))(input)?;
    let value_a = parse_value(value_a);
    let value_b = parse_value(value_b);
    let operation = match operation {
        '*' => Operation::Multiple(value_a, value_b),
        '+' => Operation::Add(value_a, value_b),
        _ => panic!("Operation {:?} not support!", operation),
    };
    Ok((input, operation))
}

fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tuple((tag("Starting items:"), space0))(input)?;
    let (input, items) = separated_list0(tuple((space0, char(','), space0)), digit1)(input)?;
    let items = items.iter().map(|item| item.parse().unwrap()).collect();
    Ok((input, items))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let monkey_line = delimited(tag("Monkey "), digit1, char(':'));
    let (input, (id, items, operation, monkey_test)) = tuple((
        delimited(space0, monkey_line, newline),
        delimited(space0, parse_items, newline),
        delimited(space0, parse_operation, newline),
        parse_monkey_test,
    ))(input)?;
    let id = id.parse().unwrap();
    Ok((input, Monkey::new(id, items, operation, monkey_test)))
}

pub fn parse_monkeys(input: &str) -> Result<Vec<Monkey>, Err<Error<&str>>> {
    let (_, monkeys) = separated_list0(newline, parse_monkey)(input)?;
    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Value;
    use shared::read_sample;

    #[test]
    fn test_parse_monkey_test() {
        let input = "Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";
        println!("{}", input);
        let result = parse_monkey_test(input).unwrap();
        let monkey_test = result.1;
        assert!(matches!(
            monkey_test,
            MonkeyTest {
                divisible_by: 23,
                if_true: 2,
                if_false: 3
            }
        ))
    }

    #[test]
    fn test_parse_operation() {
        let input = "Operation: new = old * 19";
        let result = parse_operation(input).unwrap();
        let operation = result.1;
        assert!(matches!(
            operation,
            Operation::Multiple(Value::Old, Value::Number(19))
        ))
    }

    #[test]
    fn test_parse_items() {
        let input = "Starting items: 79, 98";
        let items = parse_items(input).unwrap();
        assert_eq!(vec![79, 98], items.1)
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
";
        let (_, monkey) = parse_monkey(input).unwrap();
        let items = vec![79, 60, 97];
        assert_eq!(items, monkey.items);
        assert!(matches!(
            monkey,
            Monkey {
                id: 2,
                operation: Operation::Multiple(Value::Old, Value::Old),
                test: MonkeyTest {
                    divisible_by: 13,
                    if_true: 1,
                    if_false: 3
                },
                items: _,
                inspected_times: _,
            }
        ))
    }

    #[test]
    fn test_parse_monkeys() {
        let content = read_sample(11);
        let monkeys = parse_monkeys(&content).unwrap();
        assert_eq!(4, monkeys.len())
    }
}
