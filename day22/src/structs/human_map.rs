use std::collections::HashSet;

use super::{Direction, Point, StepType, TileType};

#[derive(Clone)]
pub struct MapFace {
    map: Vec<Vec<TileType>>,
    pub facing: Direction,
    pub point: Point<usize>,
    pub histories: Vec<Point<usize>>,
}

impl MapFace {
    fn new(map: Vec<Vec<TileType>>, point: Point<usize>) -> Self {
        Self {
            map,
            facing: Direction::Up,
            point,
            histories: Vec::new(),
        }
    }

    fn is_void(&self) -> bool {
        matches!(self.map[0][0], TileType::Void)
    }

    fn draw(&self) {
        if let TileType::Void = self.map[0][0] {
            println!("== VOID ==");
            println!();
            return;
        }
        let mut set = HashSet::new();
        self.histories.iter().for_each(|point| {
            set.insert(point.tuple());
        });
        for (y, row) in self.map.iter().enumerate() {
            let mut chars = Vec::new();
            for (x, tile_type) in row.iter().enumerate() {
                let point = set.contains(&(x, y));
                let char = match (tile_type, point) {
                    (TileType::Void, false) => '⬛',
                    (TileType::Void, true) => '🟥',
                    (TileType::Wall, false) => '🌚',
                    (TileType::Wall, true) => '🔴',
                    (TileType::Open, false) => '⬜',
                    (TileType::Open, true) => '🟧',
                };
                chars.push(char);
            }
            let size = chars.len();
            let string = String::from_iter(chars.into_iter());
            println!("{}, {}", size, string);
        }
        println!();
    }
}

impl std::fmt::Debug for MapFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let TileType::Void = self.map[0][0] {
            return write!(f, "== VOID ==");
        }
        write!(f, "{:?}\n", self.point)?;
        write!(f, "{:?}\n", self.facing)?;
        for row in self.map.iter() {
            for tile_type in row.iter() {
                let char = match tile_type {
                    TileType::Void => '⬛',
                    TileType::Wall => '🌚',
                    TileType::Open => '⬜',
                };
                write!(f, "{}", char)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

pub trait Rotate {
    fn rotate_full(&mut self);
    fn rotate_right(&mut self);
    fn rotate_left(&mut self);
}

impl Rotate for Option<MapFace> {
    fn rotate_full(&mut self) {
        match self {
            Some(map_face) => map_face.rotate_full(),
            None => (),
        }
    }

    fn rotate_right(&mut self) {
        match self {
            Some(map_face) => map_face.rotate_right(),
            None => (),
        }
    }

    fn rotate_left(&mut self) {
        match self {
            Some(map_face) => map_face.rotate_left(),
            None => (),
        }
    }
}

impl Rotate for MapFace {
    fn rotate_full(&mut self) {
        for row in self.map.iter_mut() {
            row.reverse()
        }
        self.map.reverse();
        self.facing.turn_right();
        self.facing.turn_right();
        self.histories.iter_mut().for_each(|point| {
            let size = self.map.len();
            *point = rotate_point_right(point, size);
            *point = rotate_point_right(point, size);
        })
    }

    fn rotate_right(&mut self) {
        let size = self.map.len();
        let mut new_map = Vec::new();
        for _ in 0..size {
            let mut new_row = Vec::new();
            for y in (0..size).rev() {
                let a = self.map[y].pop().unwrap();
                new_row.push(a);
            }
            new_map.push(new_row);
        }
        new_map.reverse();
        self.map = new_map;
        self.facing.turn_right();
        self.histories.iter_mut().for_each(|point| {
            let size = self.map.len();
            *point = rotate_point_right(point, size);
        })
    }

    fn rotate_left(&mut self) {
        let size = self.map.len();
        let mut new_map = Vec::new();
        for _ in 0..size {
            let mut new_row = Vec::new();
            for y in 0..size {
                let a = self.map[y].pop().unwrap();
                new_row.push(a);
            }
            new_map.push(new_row);
        }
        self.map = new_map;
        self.facing.turn_left();
        self.histories.iter_mut().for_each(|point| {
            let size = self.map.len();
            *point = rotate_point_left(point, size);
        })
    }
}

trait CubeRotate<T: Rotate> {
    fn get_faces(&mut self) -> &mut Vec<T>;

    fn cube_rotate(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => {
                self.get_faces()[1].rotate_right();
                self.get_faces()[2].rotate_full();
                self.get_faces()[4].rotate_left();
                self.get_faces()[5].rotate_full();
                self.get_faces().swap(0, 3);
                self.get_faces().swap(3, 5);
                self.get_faces().swap(5, 2);
            }
            Direction::Down => {
                self.get_faces()[2].rotate_left();
                self.get_faces()[3].rotate_right();
                self.get_faces().swap(0, 4);
                self.get_faces().swap(4, 5);
                self.get_faces().swap(5, 1);
            }
            Direction::Left => {
                self.get_faces()[1].rotate_left();
                self.get_faces()[3].rotate_full();
                self.get_faces()[4].rotate_right();
                self.get_faces()[5].rotate_full();
                self.get_faces().swap(0, 2);
                self.get_faces().swap(2, 5);
                self.get_faces().swap(5, 3);
            }
            Direction::Up => {
                self.get_faces()[2].rotate_right();
                self.get_faces()[3].rotate_left();
                self.get_faces().swap(0, 1);
                self.get_faces().swap(1, 5);
                self.get_faces().swap(5, 4);
            }
        }
    }
}

impl CubeRotate<Option<MapFace>> for MapCubeBuilder {
    fn get_faces(&mut self) -> &mut Vec<Option<MapFace>> {
        &mut self.map_faces
    }
}

#[derive(Debug)]
struct MapCubeBuilder {
    map_faces: Vec<Option<MapFace>>,
    histories: Vec<Direction>,
}

impl MapCubeBuilder {
    fn new() -> Self {
        Self {
            map_faces: vec![None; 6],
            histories: Vec::new(),
        }
    }

    fn build(self) -> MapCube {
        MapCube {
            map_faces: self.map_faces.into_iter().map(|e| e.unwrap()).collect(),
        }
    }

    fn set(&mut self, index: usize, map_face: MapFace) {
        self.map_faces[index] = Some(map_face)
    }

    fn is_complete(&self) -> bool {
        self.map_faces.iter().all(|e| e.is_some())
    }

    fn move_back(&mut self) {
        let direction = self.histories.pop().unwrap().invert();
        self.cube_rotate(&direction);
    }

    fn move_all_back(&mut self) {
        while !self.histories.is_empty() {
            let direction = self.histories.pop().unwrap().invert();
            self.cube_rotate(&direction);
        }
    }
}

#[derive(Debug)]
pub struct MapCube {
    pub map_faces: Vec<MapFace>,
}

impl MapCube {
    fn get_current_face(&mut self) -> &mut MapFace {
        &mut self.map_faces[0]
    }

    fn get_tile_type(&self, x: usize, y: usize) -> &TileType {
        &self.map_faces[0].map[y][x]
    }
}

impl CubeRotate<MapFace> for MapCube {
    fn get_faces(&mut self) -> &mut Vec<MapFace> {
        &mut self.map_faces
    }
}

pub struct HumanMap {
    face_size: usize,
    pub current_point: Point<usize>,
    pub current_facing: Direction,
    pub map_cube: MapCube,
    path: Vec<StepType>,
}

impl HumanMap {
    pub fn new(full_map: Vec<Vec<TileType>>, path: Vec<StepType>, face_size: usize) -> Self {
        let first_row = full_map.get(0).unwrap();
        let x_max = (first_row.len() / face_size) - 1;
        let y_max = (full_map.len() / face_size) - 1;
        let mut map_faces = Vec::new();
        for y in 0..=y_max {
            let mut row = Vec::new();
            for x in 0..=x_max {
                let map_face = extract_map_face(&full_map, face_size, x, y);
                row.push(map_face);
            }
            map_faces.push(row);
        }
        let map_cube = build_map_cube(x_max, map_faces, y_max);
        dbg!(&map_cube);
        Self {
            face_size,
            current_point: Point::new(0, 0),
            current_facing: Direction::Right,
            map_cube,
            path,
        }
    }

    pub fn follow_path(&mut self) {
        self.map_cube
            .get_current_face()
            .histories
            .push(Point::new(0, 0));
        for step in self.path.clone().into_iter() {
            match step {
                StepType::Forward(step_count) => self.move_forward(step_count),
                StepType::TurnLeft => self.current_facing.turn_left(),
                StepType::TurnRight => self.current_facing.turn_right(),
            };
        }
    }

    fn move_forward(&mut self, step_count: usize) {
        let mut step_count = step_count;
        println!("{} step", step_count);
        while step_count > 0 {
            let (tile_type, (x, y)) = self.get_next_tile_type();
            match tile_type {
                TileType::Void => panic!("Void is Danger!"),
                TileType::Wall => {
                    break;
                }
                TileType::Open => {
                    self.current_point.set((x, y));
                    self.map_cube
                        .get_current_face()
                        .histories
                        .push(Point::new(x, y));
                    step_count -= 1;
                    continue;
                }
            }
        }
    }

    fn get_next_tile_type(&mut self) -> (TileType, (usize, usize)) {
        let mut rotated = false;
        let point = &self.current_point;
        let (mut x, mut y) = (point.x, point.y);
        match self.current_facing {
            Direction::Up => {
                if y == 0 {
                    self.map_cube.cube_rotate(&Direction::Up);
                    rotated = true;
                    y = self.face_size - 1;
                } else {
                    y = y - 1;
                }
            }
            Direction::Right => {
                if x == (self.face_size - 1) {
                    self.map_cube.cube_rotate(&Direction::Right);
                    rotated = true;
                    x = 0;
                } else {
                    x = x + 1;
                };
            }
            Direction::Down => {
                if y == (self.face_size - 1) {
                    self.map_cube.cube_rotate(&Direction::Down);
                    rotated = true;
                    y = 0;
                } else {
                    y = y + 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    self.map_cube.cube_rotate(&Direction::Left);
                    rotated = true;
                    x = self.face_size - 1;
                } else {
                    x = x - 1;
                };
            }
        };
        let tile_type = self.map_cube.get_tile_type(x, y).clone();
        if rotated {
            if let TileType::Wall = tile_type {
                match self.current_facing {
                    Direction::Right => self.map_cube.cube_rotate(&Direction::Left),
                    Direction::Down => self.map_cube.cube_rotate(&Direction::Up),
                    Direction::Left => self.map_cube.cube_rotate(&Direction::Right),
                    Direction::Up => self.map_cube.cube_rotate(&Direction::Down),
                }
            } else {
                println!();
            }
        }
        (tile_type, (x, y))
    }

    pub fn get_result(&mut self) -> Option<usize> {
        dbg!(&self.current_point);
        dbg!(&self.current_facing);
        println!("{:?}", &self.map_cube.map_faces[0]);
        println!("================");
        self.map_cube.map_faces[1].draw();
        self.map_cube.map_faces[0].draw();
        self.map_cube.map_faces[4].draw();
        self.map_cube.map_faces[5].draw();
        self.map_cube.map_faces[2].draw();
        self.map_cube.map_faces[3].draw();

        let mut current_point = self.current_point.clone();
        let mut current_facing = self.current_facing.clone();
        let mut current_face = self.map_cube.map_faces[0].clone();
        match current_face.facing {
            Direction::Right => {
                current_face.rotate_left();
                current_facing.turn_left();
                current_point = rotate_point_left(&current_point, self.face_size);
            }
            Direction::Down => {
                current_face.rotate_left();
                current_facing.turn_left();
                current_point = rotate_point_left(&current_point, self.face_size);
                current_face.rotate_left();
                current_facing.turn_left();
                current_point = rotate_point_left(&current_point, self.face_size);
            }
            Direction::Left => {
                current_face.rotate_right();
                current_facing.turn_right();
                current_point = rotate_point_right(&current_point, self.face_size);
            }
            Direction::Up => (),
        }

        dbg!(&current_point);
        dbg!(&current_facing);
        println!("{:?}", &current_face);

        let row = current_face.point.y + current_point.y + 1;
        let column = current_face.point.x + current_point.x + 1;
        Some(1000 * row + 4 * column + current_facing as usize)
    }
}

fn rotate_point_left(point: &Point<usize>, size: usize) -> Point<usize> {
    let x = point.y;
    let y = size - 1 - point.x;
    Point::new(x, y)
}

fn rotate_point_right(point: &Point<usize>, size: usize) -> Point<usize> {
    let x = size - 1 - point.y;
    let y = point.x;
    Point::new(x, y)
}

fn build_map_cube(x_max: usize, map_faces: Vec<Vec<MapFace>>, y_max: usize) -> MapCube {
    let mut map_cube_builder = MapCubeBuilder::new();
    let mut current_point = Point::new(0, 0);
    for x in 0..=x_max {
        let y = 0;
        let map_face = &map_faces[y][x];
        if map_face.is_void() {
            continue;
        }
        map_face.draw();
        current_point.set((x, y));
        break;
    }
    let mut visited = HashSet::new();
    let mut points = vec![(current_point, None)];
    while !points.is_empty() {
        let (point, direction) = points.pop().unwrap();
        let map_face = &map_faces[point.y][point.x];
        visited.insert(point.tuple());
        if let Some(direction) = direction {
            map_cube_builder.cube_rotate(&direction);
            map_cube_builder.histories.push(direction);
        }
        map_cube_builder.set(0, map_face.clone());
        if map_cube_builder.is_complete() {
            break;
        }
        let mut new_points = Vec::new();
        if point.x < x_max {
            let new_point = Point::new(point.x + 1, point.y);
            if !visited.contains(&new_point.tuple()) {
                let map_face = &map_faces[new_point.y][new_point.x];
                if !map_face.is_void() {
                    new_points.push((new_point, Some(Direction::Right)));
                }
            }
        }
        if point.x > 0 {
            let new_point = Point::new(point.x - 1, point.y);
            if !visited.contains(&new_point.tuple()) {
                let map_face = &map_faces[new_point.y][new_point.x];
                if !map_face.is_void() {
                    new_points.push((new_point, Some(Direction::Left)));
                }
            }
        }
        if point.y < y_max {
            let new_point = Point::new(point.x, point.y + 1);
            if !visited.contains(&new_point.tuple()) {
                let map_face = &map_faces[new_point.y][new_point.x];
                if !map_face.is_void() {
                    new_points.push((new_point, Some(Direction::Down)));
                }
            }
        }
        if point.y > 0 {
            let new_point = Point::new(point.x, point.y - 1);
            if !visited.contains(&new_point.tuple()) {
                let map_face = &map_faces[new_point.y][new_point.x];
                if !map_face.is_void() {
                    new_points.push((new_point, Some(Direction::Up)));
                }
            }
        }
        if new_points.is_empty() {
            map_cube_builder.move_back();
        } else {
            points.push((point, None));
            points.append(&mut new_points);
        }
    }
    map_cube_builder.move_all_back();
    map_cube_builder.build()
}

fn extract_map_face(
    full_map: &Vec<Vec<TileType>>,
    face_size: usize,
    x: usize,
    y: usize,
) -> MapFace {
    let x_start = x * face_size;
    let x_end = x_start + face_size;
    let y_start = y * face_size;
    let y_end = y_start + face_size;
    let mut map = Vec::new();
    for y in y_start..y_end {
        let mut row = Vec::new();
        for x in x_start..x_end {
            row.push(full_map[y][x].clone())
        }
        map.push(row);
    }
    MapFace::new(map, Point::new(x_start, y_start))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! match_map_face {
        ($x:expr, $y:path) => {
            matches!($x.as_ref().unwrap().map[0][0], $y)
        };
    }

    #[test]
    fn test_map_cube_builder_move_to() {
        let mut map_cube_builder = MapCubeBuilder::new();
        map_cube_builder.set(
            0,
            MapFace::new(vec![vec![TileType::Wall]], Point::new(0, 0)),
        );
        map_cube_builder.set(
            1,
            MapFace::new(vec![vec![TileType::Open]], Point::new(0, 0)),
        );
        map_cube_builder.set(
            2,
            MapFace::new(vec![vec![TileType::Open]], Point::new(0, 0)),
        );
        map_cube_builder.set(
            3,
            MapFace::new(vec![vec![TileType::Open]], Point::new(0, 0)),
        );
        map_cube_builder.set(
            4,
            MapFace::new(vec![vec![TileType::Open]], Point::new(0, 0)),
        );
        map_cube_builder.set(
            5,
            MapFace::new(vec![vec![TileType::Void]], Point::new(0, 0)),
        );
        match_map_face!(map_cube_builder.map_faces[0], TileType::Wall);
        map_cube_builder.cube_rotate(&Direction::Down);
        match_map_face!(map_cube_builder.map_faces[1], TileType::Wall);
        match_map_face!(map_cube_builder.map_faces[4], TileType::Void);
        map_cube_builder.cube_rotate(&Direction::Up);
        match_map_face!(map_cube_builder.map_faces[0], TileType::Wall);
        match_map_face!(map_cube_builder.map_faces[5], TileType::Void);
        map_cube_builder.cube_rotate(&Direction::Left);
        match_map_face!(map_cube_builder.map_faces[3], TileType::Wall);
        match_map_face!(map_cube_builder.map_faces[2], TileType::Void);
        map_cube_builder.cube_rotate(&Direction::Right);
        match_map_face!(map_cube_builder.map_faces[0], TileType::Wall);
        match_map_face!(map_cube_builder.map_faces[5], TileType::Void);
    }

    #[test]
    fn test_rotate_for_map_face_rotate_left() {
        let mut map_face = MapFace::new(
            vec![
                vec![TileType::Wall, TileType::Open],
                vec![TileType::Open, TileType::Void],
            ],
            Point::new(0, 0),
        );
        map_face.rotate_left();
        assert!(matches!(map_face.map[1][0], TileType::Wall));
        assert!(matches!(map_face.map[0][1], TileType::Void));
    }

    #[test]
    fn test_rotate_for_map_face_rotate_right() {
        let mut map_face = MapFace::new(
            vec![
                vec![TileType::Wall, TileType::Open],
                vec![TileType::Open, TileType::Void],
            ],
            Point::new(0, 0),
        );
        map_face.rotate_right();
        assert!(matches!(map_face.map[0][1], TileType::Wall));
        assert!(matches!(map_face.map[1][0], TileType::Void));
    }

    #[test]
    fn test_rotate_for_map_face_rotate_full() {
        let mut map_face = MapFace::new(
            vec![
                vec![TileType::Wall, TileType::Open],
                vec![TileType::Open, TileType::Void],
            ],
            Point::new(0, 0),
        );
        map_face.rotate_full();
        assert!(matches!(map_face.map[1][1], TileType::Wall));
        assert!(matches!(map_face.map[0][0], TileType::Void));
    }
}
