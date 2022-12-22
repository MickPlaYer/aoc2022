use super::{Direction, Point, StepType, TileType};

pub struct MonkeyMap {
    size: (isize, isize),
    pub current_point: Point<isize>,
    pub current_facing: Direction,
    full_map: Vec<Vec<TileType>>,
    path: Vec<StepType>,
}

impl MonkeyMap {
    pub fn new(full_map: Vec<Vec<TileType>>, path: Vec<StepType>) -> Self {
        let first_row = full_map.get(0).unwrap();
        let x_size = first_row.len();
        let y_size = full_map.len();
        let x = first_row
            .iter()
            .position(|tile_type| matches!(tile_type, TileType::Open))
            .unwrap();
        let current_point = Point::new(x as isize, 0);
        let current_facing = Direction::Right;
        Self {
            size: (x_size as isize, y_size as isize),
            current_point,
            current_facing,
            full_map,
            path,
        }
    }

    pub fn follow_path(&mut self) {
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
        while step_count > 0 {
            let (tile_type, (x, y)) = self.get_next_tile_type();
            match tile_type {
                TileType::Void => panic!("Void is Danger!"),
                TileType::Wall => break,
                TileType::Open => {
                    self.current_point.set((x, y));
                    step_count -= 1;
                    continue;
                }
            }
        }
    }

    fn get_next_tile_type(&mut self) -> (&TileType, (isize, isize)) {
        let point = &self.current_point;
        let (mut x, mut y) = (point.x, point.y);
        loop {
            let next_cord = self.get_next_cord((x, y));
            x = next_cord.0;
            y = next_cord.1;
            let next_tile_type = self.get_tile_type(x, y);
            if let TileType::Open | TileType::Wall = next_tile_type {
                return (next_tile_type, (x, y));
            }
        }
    }

    fn get_next_cord(&self, cord: (isize, isize)) -> (isize, isize) {
        let (x, y) = cord;
        let (x, y) = match self.current_facing {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };
        let x = x.checked_rem_euclid(self.size.0).unwrap();
        let y = y.checked_rem_euclid(self.size.1).unwrap();
        (x, y)
    }

    fn get_tile_type(&self, x: isize, y: isize) -> &TileType {
        let x = x as usize;
        let y = y as usize;
        &self.full_map[y][x]
    }
}
