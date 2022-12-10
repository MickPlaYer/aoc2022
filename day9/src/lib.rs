use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    steps: usize,
}

impl Move {
    fn parse(line: &str) -> Self {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Can not parse direction with {}!", direction),
        };
        let steps = split.next().unwrap().parse().unwrap();
        Move { direction, steps }
    }
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn distance(&self, other: &Point) -> isize {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff.max(y_diff)
    }
}

struct Rope {
    head: Point,
    tail: Point,
    footprint: HashMap<Point, bool>,
}

impl Rope {
    fn new() -> Self {
        let tail = Point::default();
        let mut footprint = HashMap::new();
        footprint.insert(tail.clone(), true);
        Self {
            head: Point::default(),
            tail,
            footprint,
        }
    }

    fn apply_move(&mut self, a_move: &Move) {
        for _ in 0..a_move.steps {
            let previous_head = self.head.clone();
            self.head.move_to(&a_move.direction);
            if self.head.distance(&self.tail) > 1 {
                self.footprint.insert(previous_head.clone(), true);
                self.tail = previous_head;
            }
        }
    }

    fn count_footprint(&self) -> usize {
        self.footprint.iter().count()
    }
}

pub fn process_part1(content: String) -> Option<usize> {
    let moves = content
        .lines()
        .map(|line| Move::parse(line))
        .collect::<Vec<Move>>();
    let mut rope = Rope::new();
    moves.iter().for_each(|a_move| rope.apply_move(a_move));
    Some(rope.count_footprint())
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 9;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(13), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(6098), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
