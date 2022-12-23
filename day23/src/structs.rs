pub type Coordinate = (isize, isize);
pub type Boundary = (isize, isize, isize, isize);

pub struct Neighbors {
    cells: [bool; 8],
}

impl Neighbors {
    pub fn new(cells: [bool; 8]) -> Self {
        Self { cells }
    }

    pub fn is_empty(&self) -> bool {
        !self.cells.contains(&true)
    }

    pub fn have_space(&self, direction: &Direction) -> bool {
        match direction {
            Direction::North => !self.cells[6] && !self.cells[7] && !self.cells[0],
            Direction::South => !self.cells[2] && !self.cells[3] && !self.cells[4],
            Direction::West => !self.cells[4] && !self.cells[5] && !self.cells[6],
            Direction::East => !self.cells[0] && !self.cells[1] && !self.cells[2],
        }
    }
}

pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn default_directions() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
    }
}

pub struct Creature {
    x: isize,
    y: isize,
}

impl Creature {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn coordinate(&self) -> Coordinate {
        (self.x, self.y)
    }

    pub fn proposes_moving(&self, direction: &Direction) -> Proposal {
        let from = self.coordinate();
        let to = match direction {
            Direction::North => (self.x, self.y - 1),
            Direction::South => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
            Direction::East => (self.x + 1, self.y),
        };
        Proposal::new(from, to)
    }

    pub fn just_stay(&self) -> Proposal {
        let from = self.coordinate();
        Proposal::new(from, from)
    }
}

pub struct Proposal {
    from: Coordinate,
    to: Coordinate,
    rejected: bool,
}

impl Proposal {
    fn new(from: Coordinate, to: Coordinate) -> Self {
        Self {
            from,
            to,
            rejected: false,
        }
    }

    pub fn coordinate(&self) -> Coordinate {
        self.to
    }

    pub fn perform(self) -> Creature {
        let (x, y) = if self.rejected { self.from } else { self.to };
        Creature::new(x, y)
    }

    pub fn reject(&mut self) {
        self.rejected = true
    }

    pub fn is_stay(&self) -> bool {
        self.from == self.to
    }

    pub fn is_rejeted(&self) -> bool {
        self.rejected
    }
}
