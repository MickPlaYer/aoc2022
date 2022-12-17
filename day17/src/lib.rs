#[derive(Clone)]
enum Jet {
    L,
    R,
}

#[derive(Debug, Clone)]
enum Rock {
    Bar,
    Star,
    Crowbar,
    Stick,
    Block,
}

impl Rock {
    fn left_side(&self, point: &Point) -> Vec<Point> {
        match self {
            Rock::Bar => vec![point.diff(0, 0)],
            Rock::Star => vec![point.diff(1, 0), point.diff(0, 1), point.diff(1, 2)],
            Rock::Crowbar => vec![point.diff(0, 0)],
            Rock::Stick => vec![
                point.diff(0, 0),
                point.diff(0, 1),
                point.diff(0, 2),
                point.diff(0, 3),
            ],
            Rock::Block => vec![point.diff(0, 0), point.diff(0, 1)],
        }
    }

    fn right_side(&self, point: &Point) -> Vec<Point> {
        match self {
            Rock::Bar => vec![point.diff(3, 0)],
            Rock::Star => vec![point.diff(1, 0), point.diff(2, 1), point.diff(1, 2)],
            Rock::Crowbar => vec![point.diff(2, 0), point.diff(2, 1), point.diff(2, 2)],
            Rock::Stick => vec![
                point.diff(0, 0),
                point.diff(0, 1),
                point.diff(0, 2),
                point.diff(0, 3),
            ],
            Rock::Block => vec![point.diff(1, 0), point.diff(1, 1)],
        }
    }

    fn bottom_side(&self, point: &Point) -> Vec<Point> {
        match self {
            Rock::Bar => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(2, 0),
                point.diff(3, 0),
            ],
            Rock::Star => vec![point.diff(1, 0), point.diff(0, 1), point.diff(2, 1)],
            Rock::Crowbar => vec![point.diff(0, 0), point.diff(1, 0), point.diff(2, 0)],
            Rock::Stick => vec![point.diff(0, 0)],
            Rock::Block => vec![point.diff(0, 0), point.diff(1, 0)],
        }
    }

    fn all_blocks(&self, point: &Point) -> Vec<Point> {
        match self {
            Rock::Bar => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(2, 0),
                point.diff(3, 0),
            ],
            Rock::Star => vec![
                point.diff(1, 0),
                point.diff(0, 1),
                point.diff(1, 1),
                point.diff(2, 1),
                point.diff(1, 2),
            ],
            Rock::Crowbar => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(2, 0),
                point.diff(2, 1),
                point.diff(2, 2),
            ],
            Rock::Stick => vec![
                point.diff(0, 0),
                point.diff(0, 1),
                point.diff(0, 2),
                point.diff(0, 3),
            ],
            Rock::Block => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(0, 1),
                point.diff(1, 1),
            ],
        }
    }
}

struct Generator<T> {
    index: usize,
    items: Vec<T>,
}

impl<T: Clone> Generator<T> {
    fn new(items: Vec<T>) -> Self {
        Self { index: 0, items }
    }

    fn next(&mut self) -> T {
        let item = self.items[self.index].clone();
        if self.index < (self.items.len() - 1) {
            self.index += 1;
        } else {
            self.index = 0;
        }
        item
    }
}

fn parse_jets(input: &str) -> Vec<Jet> {
    input
        .chars()
        .filter_map(|char| match char {
            '>' => Some(Jet::R),
            '<' => Some(Jet::L),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn diff(&self, x: isize, y: isize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn go_left(&mut self) {
        self.x -= 1;
    }

    fn go_right(&mut self) {
        self.x += 1;
    }

    fn go_down(&mut self) {
        self.y -= 1;
    }
}

struct Hall {
    map: Vec<Vec<usize>>,
}

impl Hall {
    fn new() -> Self {
        Self {
            map: vec![Vec::new(); 7],
        }
    }

    fn top(&self) -> usize {
        self.map.iter().map(|v| v.len()).max().unwrap()
    }

    fn can_left(&self, points: &Vec<Point>) -> bool {
        points
            .iter()
            .all(|point| !self.is_blocked(&point.diff(-1, 0)))
    }

    fn can_right(&self, points: &Vec<Point>) -> bool {
        points
            .iter()
            .all(|point| !self.is_blocked(&point.diff(1, 0)))
    }

    fn can_down(&self, points: &Vec<Point>) -> bool {
        points
            .iter()
            .all(|point| !self.is_blocked(&point.diff(0, -1)))
    }

    fn rock_rest(&mut self, point: &Point, rock: &Rock) {
        let rock_mark = match rock {
            Rock::Bar => 1,
            Rock::Star => 2,
            Rock::Crowbar => 3,
            Rock::Stick => 4,
            Rock::Block => 5,
        };
        let blocks = rock.all_blocks(&point);
        for block in blocks {
            self.set_blocked(&block, rock_mark);
        }
    }

    fn is_blocked(&self, point: &Point) -> bool {
        let v = self.map.get(point.x as usize);
        if v.is_none() {
            return true;
        }
        if point.y < 0 {
            return true;
        }
        *v.unwrap().get(point.y as usize).unwrap_or(&0) > 0
    }

    fn set_blocked(&mut self, point: &Point, rock_mark: usize) {
        let v = self.map.get_mut(point.x as usize).unwrap();
        let y = point.y as usize;
        if v.len() <= y {
            while v.len() < y {
                v.push(0);
            }
            v.push(rock_mark);
        } else {
            v[y] = rock_mark;
        }
    }

    fn draw(&mut self, other_points: Vec<Point>) {
        let top = self.top() + 20;
        let number_size = top.to_string().len();
        for y in (0..top).rev() {
            print!("| {:0width$} |", y + 1, width = number_size);
            self.map.iter().enumerate().for_each(|(x, v)| {
                let a = v.get(y);
                let c = if a.is_none() {
                    let mut d = None;
                    for point in &other_points {
                        if (point.x == x as isize) && (point.y == y as isize) {
                            d = Some("x".to_string())
                        }
                    }
                    d.unwrap_or(" ".to_string())
                } else {
                    match a.unwrap() {
                        0 => ".".to_string(),
                        _ => a.unwrap().to_string(),
                    }
                };
                print!("{}", c);
            });
            print!("\n");
        }
    }
}

fn simulate_n_rocks(content: String, amount_of_rocks: usize) -> Hall {
    let mut rocks = Generator::new(vec![
        Rock::Bar,
        Rock::Star,
        Rock::Crowbar,
        Rock::Stick,
        Rock::Block,
    ]);
    let mut jets = Generator::new(parse_jets(&content));
    let mut hall = Hall::new();
    for _ in 0..amount_of_rocks {
        let rock = rocks.next();
        let mut point = Point::new(2, hall.top() as isize + 3);
        loop {
            let jet = jets.next();
            match jet {
                Jet::L => {
                    if hall.can_left(&rock.left_side(&point)) {
                        point.go_left()
                    }
                }
                Jet::R => {
                    if hall.can_right(&rock.right_side(&point)) {
                        point.go_right()
                    }
                }
            }
            if hall.can_down(&rock.bottom_side(&point)) {
                point.go_down();
            } else {
                hall.rock_rest(&point, &rock);
                break;
            }
        }
    }
    hall
}

pub fn process_part1(content: String) -> Option<usize> {
    let mut hall = simulate_n_rocks(content, 2022);
    hall.draw(vec![]);
    Some(hall.top())
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 17;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(3068), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(3147), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(1514285714288), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
