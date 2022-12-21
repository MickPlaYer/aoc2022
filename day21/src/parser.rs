use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of, space0},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::{MonkeyRecord, MonkeyType, OperationType};

fn operation_type(input: &str) -> IResult<&str, OperationType> {
    let (input, char) = delimited(space0, one_of("+-*/"), space0)(input)?;
    let operation_type = match char {
        '+' => OperationType::Add,
        '-' => OperationType::Sub,
        '*' => OperationType::Mul,
        '/' => OperationType::Div,
        _ => panic!("Unknow operation_type {}!", char),
    };
    Ok((input, operation_type))
}

fn monkey_type_operator(input: &str) -> IResult<&str, MonkeyRecord> {
    let (input, (name, (lhs, operation_type, rhs))) =
        separated_pair(alpha1, tag(": "), tuple((alpha1, operation_type, alpha1)))(input)?;
    let monkey_type = MonkeyType::Operator(operation_type, lhs.into(), rhs.into());
    Ok((input, MonkeyRecord::new(name.into(), monkey_type)))
}

fn monkey_type_yell(input: &str) -> IResult<&str, MonkeyRecord> {
    let (input, (name, number)) = separated_pair(alpha1, tag(": "), digit1)(input)?;
    let number = number.parse().unwrap();
    let monkey_type = MonkeyType::Yell(number);
    Ok((input, MonkeyRecord::new(name.into(), monkey_type)))
}

fn parse_line(input: &str) -> IResult<&str, MonkeyRecord> {
    let (input, monkey_record) = alt((monkey_type_operator, monkey_type_yell))(input)?;
    Ok((input, monkey_record))
}

pub(crate) fn parse(input: &str) -> Vec<MonkeyRecord> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}
