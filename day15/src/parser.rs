use crate::structs::{Point, Record};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::opt,
    sequence::{pair, preceded},
    IResult,
};

fn number(input: &str) -> IResult<&str, isize> {
    let (input, (sign, number)) = pair(opt(char('-')), digit1)(input)?;
    let mut number = number.parse().unwrap();
    if sign.is_some() {
        number *= -1;
    }
    Ok((input, number))
}

fn parse_line(input: &str) -> IResult<&str, [isize; 4]> {
    let (input, s_x) = preceded(tag("Sensor at x="), number)(input)?;
    let (input, s_y) = preceded(tag(", y="), number)(input)?;
    let (input, b_x) = preceded(tag(": closest beacon is at x="), number)(input)?;
    let (input, b_y) = preceded(tag(", y="), number)(input)?;
    Ok((input, [s_x, s_y, b_x, b_y]))
}

pub fn parse(input: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for line in input.lines() {
        let (_, data) = parse_line(line).unwrap();
        records.push(Record::new(
            Point::new(data[0], data[1]),
            Point::new(data[2], data[3]),
        ));
    }
    records
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let (_, data) = parse_line(input).unwrap();
        assert_eq!([2, 18, -2, 15], data)
    }
}
