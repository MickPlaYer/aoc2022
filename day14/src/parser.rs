use crate::structs::{Point, Rock};
use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list0,
    sequence::separated_pair, IResult,
};

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(digit1, tag(","), digit1)(input)?;
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    Ok((input, Point::new(x, y)))
}

fn parse_rock(input: &str) -> IResult<&str, Rock> {
    let (input, points) = separated_list0(tag(" -> "), parse_point)(input)?;
    Ok((input, Rock::new(points)))
}

pub fn parse(input: &str) -> Rock {
    let (_, rock) = parse_rock(input).unwrap();
    rock
}
