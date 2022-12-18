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

pub struct Point {
    x: usize,
    y: usize,
    z: usize,
    sides: [bool; 6],
}

impl Point {
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

    pub fn attch(&mut self, other: &mut Point) {
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
