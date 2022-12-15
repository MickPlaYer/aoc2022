use std::ops::RangeInclusive;

pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub struct Record {
    sensor: Point,
    beacon: Point,
}

impl Record {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        Self { sensor, beacon }
    }

    fn covered_distance(&self) -> usize {
        self.sensor.x.abs_diff(self.beacon.x) + self.sensor.y.abs_diff(self.beacon.y)
    }

    pub fn is_cover_line(&self, check_line: &isize) -> bool {
        let distance = self.covered_distance() as isize;
        let center = self.sensor.y;
        let range = (center - distance)..=(center + distance);
        range.contains(check_line)
    }

    pub fn covered_range_at_line(&self, check_line: &isize) -> RangeInclusive<isize> {
        let covered_distance = self.covered_distance();
        let distance_to_line = self.sensor.y.abs_diff(*check_line);
        let distance = covered_distance.abs_diff(distance_to_line) as isize;
        let center = self.sensor.x;
        (center - distance)..=(center + distance)
    }

    pub fn get_beacon(&self) -> &Point {
        &self.beacon
    }
}

#[derive(Clone, Debug)]
enum Mark {
    Empty,
    Coverd,
    Beacon,
}

pub struct Boundary {
    left: isize,
    right: isize,
    marks: Vec<Mark>,
}

impl Boundary {
    pub fn new(left: isize, right: isize) -> Self {
        let size = right.abs_diff(left);
        let marks = vec![Mark::Empty; size + 1];
        Boundary { left, right, marks }
    }

    pub fn count_coverd(&self) -> usize {
        self.marks
            .iter()
            .filter(|mark| matches!(mark, Mark::Coverd))
            .count()
    }

    pub fn set_beacon(&mut self, position: isize) {
        self.set_mark(position, Mark::Beacon);
    }

    pub fn set_coverd(&mut self, position: isize) {
        self.set_mark(position, Mark::Coverd);
    }

    fn set_mark(&mut self, position: isize, mark: Mark) {
        if !(self.left..=self.right).contains(&position) {
            panic!(
                "out of range! {}..{}, {} = {:?}",
                self.left, self.right, position, mark
            )
        }
        let real_positon = (position - self.left) as usize;
        if let Mark::Empty = self.marks[real_positon] {
            self.marks[real_positon] = mark
        }
    }
}
