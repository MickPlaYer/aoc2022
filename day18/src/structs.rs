use std::{cell::RefCell, collections::HashSet, ops::Range};

#[derive(Clone)]
pub enum SpaceType {
    Unchecked,
    Occupied(RefCell<Cube>),
    Empty,
}

pub struct Space {
    pub boundary: Boundary,
    pub space: Vec<Vec<Vec<SpaceType>>>,
}

impl Space {
    pub fn from_cubes(cubes: Vec<Cube>) -> Self {
        let boundary = Boundary::from_cubes(&cubes);
        let mut space =
            vec![
                vec![vec![SpaceType::Unchecked; boundary.max_z + 1]; boundary.max_y + 1];
                boundary.max_x + 1
            ];
        for cube in cubes {
            let x = cube.x;
            let y = cube.y;
            let z = cube.z;
            space[x][y][z] = SpaceType::Occupied(RefCell::new(cube));
        }
        Self { space, boundary }
    }

    pub fn draw(&mut self) {
        let x_width = self.boundary.x_width();
        for z in self.boundary.z_range() {
            println!(" z: {}", z);
            println!("+{:-<1$}+", "", x_width);
            for y in self.boundary.y_range() {
                print!("|");
                for x in self.boundary.x_range() {
                    let point = Point::new(x, y, z);
                    match self.get_space(&point) {
                        SpaceType::Unchecked => print!("?"),
                        SpaceType::Occupied(_) => print!("#"),
                        SpaceType::Empty => print!("."),
                    }
                }
                print!("|\n")
            }
            println!("+{:-<1$}+", "", x_width);
        }
    }

    pub fn expend_empty_from_edge(&mut self) {
        for z in self.boundary.z_range() {
            for y in self.boundary.y_range() {
                for x in self.boundary.x_range() {
                    let point = Point::new(x, y, z);
                    if let SpaceType::Unchecked = self.get_space(&point) {
                        self.fill_empty(point);
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    fn fill_empty(&mut self, point: Point) {
        let mut visited = HashSet::new();
        let mut points = vec![point];
        loop {
            if points.is_empty() {
                break;
            }
            let current = points.pop().unwrap();
            if self.boundary.is_edge(&current) {
                self.set_space(&current, SpaceType::Empty);
            }
            if let SpaceType::Empty = self.get_space(&current) {
                let neighbors = self.boundary.neighbors_from(&current);
                for neighbor in neighbors {
                    if visited.contains(&neighbor) {
                        continue;
                    }
                    if let SpaceType::Unchecked = self.get_space(&neighbor) {
                        self.set_space(&neighbor, SpaceType::Empty);
                        points.push(neighbor);
                    }
                }
            }
            visited.insert(current);
        }
    }

    fn get_space(&self, point: &Point) -> &SpaceType {
        &self.space[point.x][point.y][point.z]
    }

    fn set_space(&mut self, point: &Point, space_type: SpaceType) {
        self.space[point.x][point.y][point.z] = space_type;
    }

    pub fn fill_uncheck_with_cube(&mut self) {
        for z in self.boundary.z_range() {
            for y in self.boundary.y_range() {
                for x in self.boundary.x_range() {
                    let point = Point::new(x, y, z);
                    if let SpaceType::Unchecked = self.get_space(&point) {
                        let mut new_cube = Cube::new(x, y, z);
                        let neighbors = self.boundary.neighbors_from(&point);
                        for neighbor in neighbors {
                            if let SpaceType::Occupied(cube) = self.get_space(&neighbor) {
                                let mut cube = cube.borrow_mut();
                                new_cube.attch(&mut cube);
                            }
                        }
                        self.set_space(&point, SpaceType::Occupied(RefCell::new(new_cube)));
                    }
                }
            }
        }
    }

    pub fn get_total_face_count(&self) -> usize {
        let mut total_face_count = 0;
        for z in self.boundary.z_range() {
            for y in self.boundary.y_range() {
                for x in self.boundary.x_range() {
                    let point = Point::new(x, y, z);
                    if let SpaceType::Occupied(cube) = self.get_space(&point) {
                        total_face_count += cube.borrow().get_face_count();
                    }
                }
            }
        }
        total_face_count
    }
}

#[derive(Debug)]
pub struct Boundary {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
    pub min_z: usize,
    pub max_z: usize,
}

impl Boundary {
    pub fn from_cubes(cubes: &Vec<Cube>) -> Self {
        let mut min_x = usize::MAX;
        let mut max_x = 0;
        let mut min_y = usize::MAX;
        let mut max_y = 0;
        let mut min_z = usize::MAX;
        let mut max_z = 0;
        for cube in cubes {
            if cube.x < min_x {
                min_x = cube.x;
            }
            if cube.x > max_x {
                max_x = cube.x;
            }
            if cube.y < min_y {
                min_y = cube.y;
            }
            if cube.y > max_y {
                max_y = cube.y;
            }
            if cube.z < min_z {
                min_z = cube.z;
            }
            if cube.z > max_z {
                max_z = cube.z;
            }
        }
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    fn x_range(&self) -> Range<usize> {
        self.min_x..(self.max_x + 1)
    }

    fn y_range(&self) -> Range<usize> {
        self.min_y..(self.max_y + 1)
    }

    fn z_range(&self) -> Range<usize> {
        self.min_z..(self.max_z + 1)
    }

    fn is_edge(&self, point: &Point) -> bool {
        point.x == self.min_x
            || point.x == self.max_x
            || point.y == self.min_y
            || point.y == self.max_y
            || point.z == self.min_z
            || point.z == self.max_z
    }

    fn is_out(&self, point: &Point) -> bool {
        point.x < self.min_x
            || point.x > self.max_x
            || point.y < self.min_y
            || point.y > self.max_y
            || point.z < self.min_z
            || point.z > self.max_z
    }

    fn neighbors_from(&self, point: &Point) -> Vec<Point> {
        let points = vec![
            point.diff(1, 0, 0),
            point.diff(-1, 0, 0),
            point.diff(0, 1, 0),
            point.diff(0, -1, 0),
            point.diff(0, 0, 1),
            point.diff(0, 0, -1),
        ];
        points
            .into_iter()
            .filter_map(|point| point)
            .filter(|point| !self.is_out(&point))
            .collect::<Vec<_>>()
    }

    fn x_width(&self) -> usize {
        (self.max_x - self.min_x) + 1
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn diff(&self, x: isize, y: isize, z: isize) -> Option<Self> {
        let x = (self.x as isize) + x;
        let y = (self.y as isize) + y;
        let z = (self.z as isize) + z;
        if x < 0 || y < 0 || z < 0 {
            return None;
        }
        Some(Self {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        })
    }
}

enum Side {
    // -x
    L = 0,
    // +x
    R = 1,
    // -y
    D = 2,
    // +y
    U = 3,
    // -z
    B = 4,
    // +z
    F = 5,
}

#[derive(Clone)]
pub struct Cube {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    sides: [bool; 6],
}

impl Cube {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x,
            y,
            z,
            sides: [true; 6],
        }
    }

    pub fn get_face_count(&self) -> usize {
        let mut count = 0;
        for side in self.sides {
            if side {
                count += 1;
            }
        }
        count
    }

    pub fn attch(&mut self, other: &mut Cube) {
        let diff_x = self.x.abs_diff(other.x);
        let diff_y = self.y.abs_diff(other.y);
        let diff_z = self.z.abs_diff(other.z);
        if diff_x == 1 && diff_y == 0 && diff_z == 0 {
            if self.x < other.x {
                self.remove(Side::R);
                other.remove(Side::L);
            } else {
                self.remove(Side::L);
                other.remove(Side::R);
            }
        } else if diff_x == 0 && diff_y == 1 && diff_z == 0 {
            if self.y < other.y {
                self.remove(Side::U);
                other.remove(Side::D);
            } else {
                self.remove(Side::D);
                other.remove(Side::U);
            }
        } else if diff_x == 0 && diff_y == 0 && diff_z == 1 {
            if self.z < other.z {
                self.remove(Side::F);
                other.remove(Side::B);
            } else {
                self.remove(Side::B);
                other.remove(Side::F);
            }
        }
    }

    fn remove(&mut self, side: Side) {
        self.sides[side as usize] = false
    }
}
