use std::cell::RefCell;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<&Point> for (usize, usize) {
    fn from(point: &Point) -> Self {
        (point.x, point.y)
    }
}

#[derive(Debug)]
struct Cord {
    height: char,
    x: usize,
    y: usize,
    passed: RefCell<bool>,
    distance: RefCell<usize>,
}

impl Cord {
    fn new(height: char, x: usize, y: usize) -> Self {
        Self {
            height,
            x,
            y,
            passed: RefCell::new(false),
            distance: RefCell::new(usize::MAX),
        }
    }

    fn can_go(&self, other: &Cord) -> bool {
        self.height_check(other, |other_height, self_height| {
            other_height <= self_height + 1
        })
    }

    fn height_check<F>(&self, other: &Cord, check: F) -> bool
    where
        F: Fn(usize, usize) -> bool,
    {
        if other.is_passed() {
            return false;
        }
        match other.height {
            'a'..='z' => {
                let self_height = self.height as usize;
                let other_height = other.height as usize;
                check(other_height, self_height)
            }
            _ => panic!("Not allow height {}!", other.height),
        }
    }

    fn pass(&self) {
        *self.passed.borrow_mut() = true;
    }

    fn is_passed(&self) -> bool {
        *self.passed.borrow()
    }

    fn set_distance(&self, distance: usize) {
        *self.distance.borrow_mut() = distance
    }

    fn get_distance(&self) -> usize {
        *self.distance.borrow()
    }

    fn reset(&self) {
        *self.distance.borrow_mut() = usize::MAX;
        *self.passed.borrow_mut() = false;
    }
}

impl From<&Cord> for (usize, usize) {
    fn from(cord: &Cord) -> Self {
        (cord.x, cord.y)
    }
}

struct HeightMap {
    grid: Vec<Vec<Cord>>,
    start_point: Point,
    end_point: Point,
}

impl HeightMap {
    fn parse(content: String) -> Self {
        let mut start_point = None;
        let mut end_point = None;
        let mut grid = Vec::new();
        for (y, line) in content.lines().enumerate() {
            let mut row = Vec::new();
            for (x, char) in line.chars().enumerate() {
                let char = Self::convert_char_to_cord(char, x, y, &mut start_point, &mut end_point);
                row.push(char);
            }
            grid.push(row);
        }
        let start_point = start_point.unwrap();
        let end_point = end_point.unwrap();
        Self {
            start_point,
            end_point,
            grid,
        }
    }

    fn convert_char_to_cord(
        char: char,
        x: usize,
        y: usize,
        start_point: &mut Option<Point>,
        end_point: &mut Option<Point>,
    ) -> Cord {
        let new_cord = match char {
            'S' => {
                let new_cord = Cord::new('a', x, y);
                *start_point = Some(Point::new(x, y));
                new_cord
            }
            'E' => {
                let new_cord = Cord::new('z', x, y);
                *end_point = Some(Point::new(x, y));
                new_cord
            }
            'a'..='z' => Cord::new(char, x, y),
            _ => panic!("Not allow char {}", char),
        };
        new_cord
    }

    fn search_path_dijkstra(&self) -> usize {
        let mut history = Vec::new();
        let current = self.from_start_point();
        current.set_distance(0);
        history.push(current);
        loop {
            history.sort_by(|a, b| b.get_distance().cmp(&a.get_distance()));
            let current = history.pop();
            if current.is_none() {
                return usize::MAX;
            }
            let current = current.unwrap();
            let neighbors = self.look_neighbors(current);
            for neighbor in neighbors {
                if neighbor.is_passed() {
                    continue;
                }
                if current.can_go(neighbor) {
                    let new_distance = current.get_distance() + 1;
                    if new_distance < neighbor.get_distance() {
                        neighbor.set_distance(new_distance);
                        history.push(neighbor);
                    }
                }
            }
            current.pass();
            if self.from_end_point().is_passed() {
                break;
            }
        }
        self.from_end_point().get_distance()
    }

    fn look_neighbors(&self, current_point: &Cord) -> Vec<&Cord> {
        let mut neighbors = Vec::new();
        let x = current_point.x;
        let y = current_point.y;
        if x > 0 {
            neighbors.push(self.from_point((x - 1, y)));
        }
        neighbors.push(self.from_point((x + 1, y)));
        if y > 0 {
            neighbors.push(self.from_point((x, y - 1)));
        }
        neighbors.push(self.from_point((x, y + 1)));
        neighbors
            .into_iter()
            .filter_map(std::convert::identity)
            .collect()
    }

    fn from_start_point(&self) -> &Cord {
        self.from_point(&self.start_point).unwrap()
    }

    fn from_end_point(&self) -> &Cord {
        self.from_point(&self.end_point).unwrap()
    }

    fn from_point(&self, point: impl Into<(usize, usize)>) -> Option<&Cord> {
        let (x, y) = point.into();
        let row = self.grid.get(y)?;
        Some(row.get(x)?)
    }

    fn all_points_with_height(&self, height: char) -> Vec<Point> {
        self.grid
            .iter()
            .flatten()
            .filter(|cord| cord.height == height)
            .map(|e| Point::new(e.x, e.y))
            .collect()
    }

    fn reset(&self) {
        self.grid.iter().flatten().for_each(Cord::reset);
    }
}

pub fn process_part1(content: String) -> Option<usize> {
    let height_map = HeightMap::parse(content);
    Some(height_map.search_path_dijkstra())
}

pub fn process_part2(content: String) -> Option<usize> {
    let mut height_map = HeightMap::parse(content);
    let points = height_map.all_points_with_height('a');
    let mut distance_list = points
        .into_iter()
        .map(|point| {
            height_map.start_point = point;
            height_map.reset();
            height_map.search_path_dijkstra()
        })
        .collect::<Vec<usize>>();
    distance_list.sort();
    distance_list.first().copied()
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 12;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(31), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(408), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(29), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(399), answer);
    }
}
