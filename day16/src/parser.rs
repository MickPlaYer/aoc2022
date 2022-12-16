use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    multi::{count, separated_list1},
    sequence::preceded,
    IResult,
};

use crate::structs::Valve;

fn valve_name(input: &str) -> IResult<&str, String> {
    let a_to_z = ('A'..='Z').collect::<String>();
    let (input, name) = count(one_of(a_to_z.as_str()), 2)(input)?;
    Ok((input, String::from_iter(name)))
}

fn parse_line(input: &str) -> IResult<&str, (String, usize, Vec<String>)> {
    let (input, name) = preceded(tag("Valve "), valve_name)(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), digit1)(input)?;
    let flow_rate = flow_rate.parse().unwrap();
    let (input, tunnels) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), valve_name),
    )(input)?;
    Ok((input, (name, flow_rate, tunnels)))
}

pub fn parse(input: &str) -> Valve {
    let (_, (name, flow_rate, tunnels)) = parse_line(input).unwrap();
    Valve::new(name, flow_rate, tunnels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let (_, (name, flow_rate, tunnels)) = parse_line(input).unwrap();
        assert_eq!("AA", name);
        assert_eq!(0, flow_rate);
        assert_eq!(vec!["DD", "II", "BB"], tunnels);
    }

    #[test]
    fn test_parse_line_2() {
        let input = "Valve HH has flow rate=22; tunnel leads to valve GG";
        let (_, (name, flow_rate, tunnels)) = parse_line(input).unwrap();
        assert_eq!("HH", name);
        assert_eq!(22, flow_rate);
        assert_eq!(vec!["GG"], tunnels);
    }
}
