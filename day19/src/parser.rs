use crate::structs::{Blueprint, BlueprintBuilder};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_ore_robot_costs(input: &str) -> IResult<&str, usize> {
    let (input, ore_robot_costs) =
        delimited(tag("Each ore robot costs "), digit1, tag(" ore. "))(input)?;
    let ore_robot_costs = ore_robot_costs.parse().unwrap();
    Ok((input, ore_robot_costs))
}

fn parse_clay_robot_costs(input: &str) -> IResult<&str, usize> {
    let (input, clay_robot_costs) =
        delimited(tag("Each clay robot costs "), digit1, tag(" ore. "))(input)?;
    let clay_robot_costs = clay_robot_costs.parse().unwrap();
    Ok((input, clay_robot_costs))
}

fn parse_obsidian_robot_cost(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (ore_cost, clay_cost)) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(digit1, tag(" ore and "), digit1),
        tag(" clay. "),
    )(input)?;
    let ore_cost = ore_cost.parse().unwrap();
    let clay_cost = clay_cost.parse().unwrap();
    Ok((input, (ore_cost, clay_cost)))
}

fn parse_geode_robot_cost(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (ore_cost, obsidian_cost)) = delimited(
        tag("Each geode robot costs "),
        separated_pair(digit1, tag(" ore and "), digit1),
        tag(" obsidian."),
    )(input)?;
    let ore_cost = ore_cost.parse().unwrap();
    let obsidian_cost = obsidian_cost.parse().unwrap();
    Ok((input, (ore_cost, obsidian_cost)))
}

fn parse_line(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), digit1, tag(": "))(input)?;
    let (input, ore_robot_costs) = parse_ore_robot_costs(input)?;
    let (input, clay_robot_costs) = parse_clay_robot_costs(input)?;
    let (input, obsidian_robot_cost) = parse_obsidian_robot_cost(input)?;
    let (input, geode_robot_cost) = parse_geode_robot_cost(input)?;
    let id = id.parse().unwrap();
    let blueprint = BlueprintBuilder::new(id)
        .set_ore_robot_cost(ore_robot_costs)
        .set_clay_robot_cost(clay_robot_costs)
        .set_obsidian_robot_cost(obsidian_robot_cost.0, obsidian_robot_cost.1)
        .set_geode_robot_cost(geode_robot_cost.0, geode_robot_cost.1)
        .build();
    Ok((input, blueprint))
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let (_, blueprint) = parse_line(line).unwrap();
        blueprints.push(blueprint);
    }
    blueprints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Blueprint 1: \
        Each ore robot costs 4 ore. \
        Each clay robot costs 2 ore. \
        Each obsidian robot costs 3 ore and 14 clay. \
        Each geode robot costs 2 ore and 7 obsidian.";
        println!("{}", line);
        let (_, blueprint) = parse_line(line).unwrap();
        assert_eq!(1, blueprint.id);
        assert_eq!(4, blueprint.ore_robot_cost);
        assert_eq!(2, blueprint.clay_robot_cost);
        assert_eq!((3, 14), blueprint.obsidian_robot_cost);
        assert_eq!((2, 7), blueprint.geode_robot_cost);
    }

    #[test]
    fn test_parse_line_2() {
        let line = "Blueprint 2: \
        Each ore robot costs 2 ore. \
        Each clay robot costs 3 ore. \
        Each obsidian robot costs 3 ore and 8 clay. \
        Each geode robot costs 3 ore and 12 obsidian.";
        println!("{}", line);
        let (_, blueprint) = parse_line(line).unwrap();
        assert_eq!(2, blueprint.id);
        assert_eq!(2, blueprint.ore_robot_cost);
        assert_eq!(3, blueprint.clay_robot_cost);
        assert_eq!((3, 8), blueprint.obsidian_robot_cost);
        assert_eq!((3, 12), blueprint.geode_robot_cost);
    }
}
