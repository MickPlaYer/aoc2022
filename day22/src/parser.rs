use crate::structs::{HumanMap, MonkeyMap, StepType, TileType};
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, multi::many1, IResult};

fn parse_full_map(lines: &mut std::str::Lines) -> Vec<Vec<TileType>> {
    let mut full_map = Vec::new();
    let mut x_size = 0;
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let mut row = Vec::new();
        for char in line.chars() {
            let tile_type = match char {
                ' ' => TileType::Void,
                '#' => TileType::Wall,
                '.' => TileType::Open,
                _ => panic!("Unknow Tile Type: {}!", char),
            };
            row.push(tile_type);
        }
        if row.len() > x_size {
            x_size = row.len();
        }
        full_map.push(row);
    }
    let mut full_map = full_map;
    for row in full_map.iter_mut() {
        while row.len() < x_size {
            row.push(TileType::Void);
        }
    }
    for row in full_map.iter() {
        for tile_type in row.iter() {
            let char = match tile_type {
                TileType::Void => '⬛',
                TileType::Wall => '🌚',
                TileType::Open => '⬜',
            };
            print!("{}", char);
        }
        println!();
    }
    println!();
    full_map
}

fn parse_step(input: &str) -> IResult<&str, StepType> {
    let (input, step) = alt((digit1, tag("L"), tag("R")))(input)?;
    let step = match step {
        "L" => StepType::TurnLeft,
        "R" => StepType::TurnRight,
        _ => StepType::Forward(step.parse().unwrap()),
    };
    Ok((input, step))
}

fn parse_path(input: &str) -> IResult<&str, Vec<StepType>> {
    let (input, step) = many1(parse_step)(input)?;
    Ok((input, step))
}

pub fn parse_part1(input: &str) -> MonkeyMap {
    let mut lines = input.lines();
    let full_map = parse_full_map(&mut lines);
    let (_, path) = parse_path(lines.next().unwrap()).unwrap();
    MonkeyMap::new(full_map, path)
}

pub fn parse_part2(input: &str, face_size: usize) -> HumanMap {
    let mut lines = input.lines();
    let full_map = parse_full_map(&mut lines);
    let (_, path) = parse_path(lines.next().unwrap()).unwrap();
    HumanMap::new(full_map, path, face_size)
}
