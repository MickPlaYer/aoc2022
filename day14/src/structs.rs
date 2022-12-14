use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

pub struct Rock {
    points: Vec<Point>,
}

impl Rock {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn block_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let mut last_point = None;
        for point in &self.points {
            if let None = last_point {
                points.push(point.clone());
                last_point = Some(point);
                continue;
            }
            generate_wall(last_point, point, &mut points);
            last_point = Some(point);
        }
        points
    }
}

fn generate_wall(last_point: Option<&Point>, point: &Point, points: &mut Vec<Point>) {
    let last_point = last_point.unwrap();
    if last_point.x == point.x {
        let range = if last_point.y < point.y {
            (last_point.y + 1)..=point.y
        } else {
            point.y..=(last_point.y - 1)
        };
        for y in range {
            points.push(Point::new(point.x, y))
        }
    }
    if last_point.y == point.y {
        let range = if last_point.x < point.x {
            (last_point.x + 1)..=point.x
        } else {
            point.x..=(last_point.x - 1)
        };
        for x in range {
            points.push(Point::new(x, point.y))
        }
    }
}

pub struct BigMap {
    sand: Point,
    rocks: Vec<Rock>,
    map: HashMap<Point, bool>,
    boundary: (usize, usize, usize, usize),
}

impl BigMap {
    pub fn new(rocks: Vec<Rock>) -> Self {
        let sand = Point::new(500, 0);
        let mut map = HashMap::new();
        let mut boundary = (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
        for rock in &rocks {
            for point in rock.block_points() {
                extend_boundary(point, &mut boundary);
                map.insert(point, true);
            }
        }
        BigMap {
            sand,
            rocks,
            map,
            boundary,
        }
    }

    fn bottom(&self) -> usize {
        self.boundary.3
    }

    pub fn drop_sand(&mut self) -> bool {
        let mut sand = self.sand.clone();
        loop {
            if sand.y > self.bottom() {
                return false;
            }
            if !self.map.get(&sand.down()).unwrap_or(&false) {
                sand = sand.down();
                continue;
            }
            if !self.map.get(&sand.down_left()).unwrap_or(&false) {
                sand = sand.down_left();
                continue;
            }
            if !self.map.get(&sand.down_right()).unwrap_or(&false) {
                sand = sand.down_right();
                continue;
            }
            self.map.insert(sand, true);
            return true;
        }
    }
}

fn extend_boundary(point: Point, boundary: &mut (usize, usize, usize, usize)) {
    let (min_x, min_y, max_x, max_y) = boundary;
    if point.x < *min_x {
        *min_x = point.x
    }
    if point.x > *max_x {
        *max_x = point.x
    }
    if point.y < *min_y {
        *min_y = point.y
    }
    if point.y > *max_y {
        *max_y = point.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rock_block_points() {
        let rock = Rock::new(vec![Point::new(2, 4), Point::new(8, 4)]);
        let mut points = rock.block_points();
        assert_eq!(7, points.len());
        assert!(points.iter().all(|point| point.y == 4));
        assert_eq!(8, points.pop().unwrap().x);
        assert_eq!(7, points.pop().unwrap().x);
        assert_eq!(6, points.pop().unwrap().x);
        assert_eq!(5, points.pop().unwrap().x);
        assert_eq!(4, points.pop().unwrap().x);
        assert_eq!(3, points.pop().unwrap().x);
        assert_eq!(2, points.pop().unwrap().x);
    }

    #[test]
    fn test_rock_block_points_2() {
        let rock = Rock::new(vec![
            Point::new(3, 3),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(2, 3),
        ]);
        let points = rock.block_points();
        assert_eq!(rock.points, points);
    }

    #[test]
    fn test_big_map_new_boundary() {
        let big_map = BigMap::new(vec![
            Rock::new(vec![Point::new(200, 200), Point::new(600, 200)]),
            Rock::new(vec![Point::new(100, 1000), Point::new(100, 100)]),
        ]);
        assert_eq!((100, 100, 600, 1000), big_map.boundary);
    }
}
