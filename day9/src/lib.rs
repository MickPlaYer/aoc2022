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
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn distance(&self, other: &Point) -> isize {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff.max(y_diff)
    }

    fn pull(&self, other: &mut Point) {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        if x_diff.is_positive() {
            other.x += 1
        } else if x_diff.is_negative() {
            other.x -= 1
        };
        if y_diff.is_positive() {
            other.y += 1
        } else if y_diff.is_negative() {
            other.y -= 1
        };
    }
}

struct Rope {
    knots: Vec<Point>,
    footprint: HashMap<Point, bool>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let knots = vec![Point::default(); size];
        let mut footprint = HashMap::new();
        footprint.insert(Point::default(), true);
        Self { knots, footprint }
    }

    fn apply_move(&mut self, a_move: &Move) {
        for _ in 0..a_move.steps {
            let (mut current, others) = self.knots.split_first_mut().unwrap();
            current.move_to(&a_move.direction);
            for next in others {
                if current.distance(next) > 1 {
                    current.pull(next);
                }
                current = next;
            }
            let tail = self.knots.last().unwrap();
            self.footprint.insert(tail.clone(), true);
        }
    }

    fn count_footprint(&self) -> usize {
        self.footprint.iter().count()
    }
}

fn simulate_rope(content: String, length: usize) -> Rope {
    let moves = content
        .lines()
        .map(|line| Move::parse(line))
        .collect::<Vec<Move>>();
    let mut rope = Rope::new(length);
    moves.iter().for_each(|a_move| rope.apply_move(a_move));
    rope
}

pub fn process_part1(content: String) -> Option<usize> {
    let rope = simulate_rope(content, 2);
    Some(rope.count_footprint())
}

pub fn process_part2(content: String) -> Option<usize> {
    let rope = simulate_rope(content, 10);
    Some(rope.count_footprint())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 9;
    use super::*;
    use shared::{read_input, read_sample};
    use test_case::test_case;

    #[test_case(( 1,  2), ( 1,  1); "From Clock  1")]
    #[test_case(( 2,  1), ( 1,  1); "From Clock  2")]
    #[test_case(( 2,  0), ( 1,  0); "From Clock  3")]
    #[test_case(( 2, -1), ( 1, -1); "From Clock  4")]
    #[test_case(( 1, -2), ( 1, -1); "From Clock  5")]
    #[test_case(( 0, -2), ( 0, -1); "From Clock  6")]
    #[test_case((-1, -2), (-1, -1); "From Clock  7")]
    #[test_case((-2, -1), (-1, -1); "From Clock  8")]
    #[test_case((-2,  0), (-1,  0); "From Clock  9")]
    #[test_case((-2,  1), (-1,  1); "From Clock 10")]
    #[test_case((-1,  2), (-1,  1); "From Clock 11")]
    #[test_case(( 0,  2), ( 0,  1); "From Clock 12")]
    fn test_point_pull((x, y): (isize, isize), (x2, y2): (isize, isize)) {
        let mut point_a = Point { x: 0, y: 0 };
        let point_b = Point { x, y };
        point_b.pull(&mut point_a);
        assert_eq!(Point { x: x2, y: y2 }, point_a);
    }

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
        assert_eq!(Some(1), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(2597), answer);
    }
}
