mod human_map;
mod monkey_map;

use std::fmt::Debug;

pub use human_map::HumanMap;
pub use monkey_map::MonkeyMap;

#[derive(Debug, Clone)]
pub enum TileType {
    Void,
    Wall,
    Open,
}

#[derive(Clone)]
pub enum StepType {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    pub fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Debug> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn set(&mut self, x_and_y: (T, T)) {
        let (x, y) = x_and_y;
        println!("{:?}, {:?}", x, y);
        self.x = x;
        self.y = y;
    }

    fn tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({:?}, {:?})", &self.x, &self.y)
    }
}
