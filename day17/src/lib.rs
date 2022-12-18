enum Jet {
    L,
    R,
}

enum RockType {
    Bar,
    Star,
    Crowbar,
    Stick,
    Block,
}

impl RockType {
    fn all_blocks(&self, point: &Point) -> Vec<Point> {
        match self {
            RockType::Bar => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(2, 0),
                point.diff(3, 0),
            ],
            RockType::Star => vec![
                point.diff(1, 0),
                point.diff(0, 1),
                point.diff(1, 1),
                point.diff(2, 1),
                point.diff(1, 2),
            ],
            RockType::Crowbar => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(2, 0),
                point.diff(2, 1),
                point.diff(2, 2),
            ],
            RockType::Stick => vec![
                point.diff(0, 0),
                point.diff(0, 1),
                point.diff(0, 2),
                point.diff(0, 3),
            ],
            RockType::Block => vec![
                point.diff(0, 0),
                point.diff(1, 0),
                point.diff(0, 1),
                point.diff(1, 1),
            ],
        }
    }

    fn get_width(&self) -> usize {
        match self {
            RockType::Bar => 4,
            RockType::Star => 3,
            RockType::Crowbar => 3,
            RockType::Stick => 1,
            RockType::Block => 2,
        }
    }
}

struct Generator<T> {
    index: usize,
    items: Vec<T>,
    size: usize,
}

impl<T> Generator<T> {
    fn new(items: Vec<T>) -> Self {
        let size = items.len() - 1;
        Self {
            index: 0,
            items,
            size,
        }
    }

    fn next(&mut self) -> &T {
        let item = &self.items[self.index];
        if self.index < self.size {
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

    fn go_up(&mut self) {
        self.y += 1;
    }
}

struct Hall {
    map: [Vec<bool>; 7],
    heighs: [usize; 7],
    ignored_heigh: usize,
}

impl Hall {
    fn new() -> Self {
        Self {
            map: Default::default(),
            heighs: [0; 7],
            ignored_heigh: 0,
        }
    }

    fn top(&self) -> usize {
        self.heighs.iter().max().unwrap() + self.ignored_heigh
    }

    fn can_fit(&self, points: &Vec<Point>) -> bool {
        !points.iter().any(|point| self.is_blocked(&point))
    }

    fn is_blocked(&self, point: &Point) -> bool {
        if point.x < 0 || point.x >= 7 {
            return true;
        }
        let y = point.y - (self.ignored_heigh as isize);
        let x = point.x as usize;
        if y < 0 {
            return true;
        }
        let y = y as usize;
        if (y + 1) > self.heighs[x] {
            return false;
        }
        self.map[x][y]
    }

    fn rock_rest(&mut self, blocks: &Vec<Point>) {
        for block in blocks {
            self.set_blocked(&block);
        }
    }

    fn set_blocked(&mut self, point: &Point) {
        let x = point.x as usize;
        let y = point.y as usize - self.ignored_heigh;
        let new_heigh = y + 1;
        if new_heigh > self.heighs[x] {
            for _ in 0..(new_heigh - self.heighs[x] - 1) {
                self.map[x].push(false)
            }
            self.map[x].push(true);
            self.heighs[x] = new_heigh
        } else {
            self.map[x][y] = true;
        }
    }

    fn ignore_bottom(&mut self) {
        const MAP_LIMIT: usize = 2000000;
        const LIMIT: usize = MAP_LIMIT / 2;
        const CUT: usize = MAP_LIMIT / 2;
        if self.heighs.iter().all(|heigh| *heigh > LIMIT) {
            let mut new_map: [Vec<bool>; 7] = Default::default();
            for (x, v) in self.map.iter().enumerate() {
                let (_, new_v) = v.split_at(CUT);
                for y in 0..new_v.len() {
                    new_map[x].push(new_v[y]);
                }
            }
            self.map = new_map;
            self.heighs
                .iter_mut()
                .for_each(|heigh| *heigh = *heigh - CUT);
            self.ignored_heigh += CUT;
        }
    }
}

struct Rock {
    anchor: Point,
    points: Vec<Point>,
    width: usize,
}

impl Rock {
    fn new(rock_type: &RockType, anchor: Point) -> Self {
        let points = rock_type.all_blocks(&anchor);
        let width = rock_type.get_width();
        Self {
            anchor,
            points,
            width,
        }
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    fn move_left(&mut self) {
        self.anchor.go_left();
        self.points.iter_mut().for_each(Point::go_left);
    }

    fn move_right(&mut self) {
        self.anchor.go_right();
        self.points.iter_mut().for_each(Point::go_right);
    }

    fn move_up(&mut self) {
        self.anchor.go_up();
        self.points.iter_mut().for_each(Point::go_up);
    }

    fn move_down(&mut self) {
        self.anchor.go_down();
        self.points.iter_mut().for_each(Point::go_down);
    }

    fn hit_left_wall(&self) -> bool {
        self.anchor.x == 0
    }

    fn hit_right_wall(&self) -> bool {
        (self.anchor.x + self.width as isize) >= 7
    }
}

fn simulate_n_rocks(content: String, amount_of_rocks: usize) -> Hall {
    let mut rock_types = Generator::new(vec![
        RockType::Bar,
        RockType::Star,
        RockType::Crowbar,
        RockType::Stick,
        RockType::Block,
    ]);
    let mut jets = Generator::new(parse_jets(&content));
    let sad = (rock_types.size + 1) * (jets.size + 1) * 345;
    dbg!(sad, rock_types.size, jets.size);
    let mut hall = Hall::new();
    #[cfg(any(finding_repeat = "sample", finding_repeat = "input"))]
    let mut last_top = 0;
    for _i in 0..amount_of_rocks {
        let rock_type = rock_types.next();
        let point = Point::new(2, hall.top() as isize + 3);
        let mut free_step = 0;
        let mut rock = Rock::new(rock_type, point);
        loop {
            free_step += 1;
            let jet = jets.next();
            match jet {
                Jet::L => {
                    if free_step < 3 {
                        if !rock.hit_left_wall() {
                            rock.move_left();
                        }
                    } else {
                        rock.move_left();
                        if !hall.can_fit(&rock.get_points()) {
                            rock.move_right();
                        }
                    }
                }
                Jet::R => {
                    if free_step < 3 {
                        if !rock.hit_right_wall() {
                            rock.move_right();
                        }
                    } else {
                        rock.move_right();
                        if !hall.can_fit(&rock.get_points()) {
                            rock.move_left();
                        }
                    }
                }
            }
            rock.move_down();
            if free_step >= 3 && !hall.can_fit(&rock.get_points()) {
                rock.move_up();
                hall.rock_rest(&rock.get_points());
                hall.ignore_bottom();
                #[cfg(any(finding_repeat = "sample", finding_repeat = "input"))]
                log_for_finding_repeat(jets.index, &hall, &mut last_top, _i + 1);
                break;
            }
        }
    }
    hall
}

#[cfg(any(finding_repeat = "sample"))]
const TEST_INDEX: usize = 1;
#[cfg(any(finding_repeat = "input"))]
const TEST_INDEX: usize = 0;

#[cfg(any(finding_repeat = "sample", finding_repeat = "input"))]
fn log_for_finding_repeat(
    jets_index: usize,
    hall: &Hall,
    last_top: &mut usize,
    rock_number: usize,
) {
    // tweak TEST_INDEX for finding repeat or maybe just log by all jets_index?
    if jets_index == TEST_INDEX {
        let next_top = hall.top();
        let diff = next_top - *last_top;
        println!("{} ({} => {}), {}", diff, next_top, last_top, rock_number);
        *last_top = next_top;
    }
}

pub fn process_part1(content: String) -> Option<usize> {
    let hall = simulate_n_rocks(content, 2022);
    Some(hall.top())
}

pub fn process_part2(
    content: String,
    before_repeat_rock_amount: usize,
    repeat_rock_amount: usize,
    repeat_height: usize,
) -> Option<usize> {
    let mut rock_amount = 1000000000000usize;
    let mut repeated_height = 0;
    while rock_amount > before_repeat_rock_amount {
        rock_amount -= repeat_rock_amount;
        repeated_height += repeat_height;
    }
    let hall = simulate_n_rocks(content, rock_amount);
    Some(repeated_height + hall.top())
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
        let multiple = 10000;
        let answer = process_part2(content, 23 + 35 * multiple, 35 * multiple, 53 * multiple);
        assert_eq!(Some(1514285714288), answer);
    }

    const BEFORE_REPEAT_ROCK_AMOUNT: usize = 1731;
    const BEFORE_REPEAT_HEIGHT: usize = 2732;
    const REPEAT_ROCK_AMOUNT: usize = 1710;
    const REPEAT_HEIGHT: usize = 2620;
    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(
            content,
            BEFORE_REPEAT_ROCK_AMOUNT,
            REPEAT_ROCK_AMOUNT,
            REPEAT_HEIGHT,
        );
        assert_eq!(Some(1532163742758), answer);
    }

    #[test]
    fn assert_repeat_data() {
        let content = read_input(DAY_NUMBER);
        let hall = simulate_n_rocks(content, BEFORE_REPEAT_ROCK_AMOUNT);
        assert_eq!(BEFORE_REPEAT_HEIGHT, hall.top());
    }

    #[test]
    fn assert_repeat_data_2() {
        let content = read_input(DAY_NUMBER);
        let hall = simulate_n_rocks(content, BEFORE_REPEAT_ROCK_AMOUNT + REPEAT_ROCK_AMOUNT);
        assert_eq!(BEFORE_REPEAT_HEIGHT + REPEAT_HEIGHT, hall.top());
    }

    #[test]
    fn assert_repeat_data_3() {
        let any_number = 314;
        let content = read_input(DAY_NUMBER);
        let hall = simulate_n_rocks(
            content,
            BEFORE_REPEAT_ROCK_AMOUNT + REPEAT_ROCK_AMOUNT * any_number,
        );
        assert_eq!(
            BEFORE_REPEAT_HEIGHT + REPEAT_HEIGHT * any_number,
            hall.top()
        );
    }

    #[test]
    fn assert_repeat_data_4() {
        let any_number = 314;
        let (extra_rocks, height_increased_by_extra_rocks) = (777, 1172);
        let content = read_input(DAY_NUMBER);
        let hall = simulate_n_rocks(
            content.clone(),
            BEFORE_REPEAT_ROCK_AMOUNT + REPEAT_ROCK_AMOUNT * any_number + extra_rocks,
        );
        let hall_2 = simulate_n_rocks(content.clone(), BEFORE_REPEAT_ROCK_AMOUNT + extra_rocks);
        assert_eq!(
            BEFORE_REPEAT_HEIGHT + REPEAT_HEIGHT * any_number + height_increased_by_extra_rocks,
            hall.top()
        );
        assert_eq!(
            BEFORE_REPEAT_HEIGHT + height_increased_by_extra_rocks,
            hall_2.top()
        );
    }

    #[test]
    #[cfg(finding_repeat = "input")]
    fn try_find_some_repeat() {
        let content = read_input(DAY_NUMBER);
        simulate_n_rocks(content, 3 * 100000);
        assert!(false);
    }

    #[test]
    #[cfg(finding_repeat = "sample")]
    fn try_find_some_repeat() {
        let content = read_sample(DAY_NUMBER);
        simulate_n_rocks(content, 3 * 1000);
        assert!(false);
    }
}
