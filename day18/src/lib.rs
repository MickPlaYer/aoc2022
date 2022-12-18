mod parser;
mod structs;

use parser::parse_line;
use structs::{Cube, Space};

fn get_attched_cubes(content: String) -> Vec<Cube> {
    let mut cubes = Vec::new();
    for line in content.lines() {
        let (x, y, z) = parse_line(line);
        let mut new_cube = Cube::new(x, y, z);
        for cube in cubes.iter_mut() {
            new_cube.attch(cube);
        }
        cubes.push(new_cube);
    }
    cubes
}

pub fn process_part1(content: String) -> Option<usize> {
    let cubes = get_attched_cubes(content);
    let result = cubes
        .into_iter()
        .map(|cube| cube.get_face_count())
        .sum::<usize>();
    Some(result)
}

pub fn process_part2(content: String) -> Option<usize> {
    let cubes = get_attched_cubes(content);
    let mut space = Space::from_cubes(cubes);
    space.expend_empty_from_edge();
    space.draw();
    space.fill_uncheck_with_cube();
    println!("=== after ===");
    space.draw();
    let result = space.get_total_face_count();
    Some(result)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 18;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(64), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(3470), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(58), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(1986), answer);
    }
}
