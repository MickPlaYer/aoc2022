use crate::structs::{Boundary, Coordinate, Creature, Neighbors};
use std::collections::HashMap;

pub fn parse_map(content: String) -> HashMap<Coordinate, Creature> {
    let mut map = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '#' {
                continue;
            }
            let creature = Creature::new(x as isize, y as isize);
            map.insert(creature.coordinate(), creature);
        }
    }
    map
}

pub fn find_neighbors(map: &HashMap<Coordinate, Creature>, coordinate: Coordinate) -> Neighbors {
    let (x, y) = coordinate;
    let mut cells = [false; 8];
    cells[0] = map.contains_key(&(x + 1, y - 1));
    cells[1] = map.contains_key(&(x + 1, y));
    cells[2] = map.contains_key(&(x + 1, y + 1));
    cells[3] = map.contains_key(&(x, y + 1));
    cells[4] = map.contains_key(&(x - 1, y + 1));
    cells[5] = map.contains_key(&(x - 1, y));
    cells[6] = map.contains_key(&(x - 1, y - 1));
    cells[7] = map.contains_key(&(x, y - 1));
    Neighbors::new(cells)
}

pub fn count_empty_grounds(map: &HashMap<Coordinate, Creature>) -> usize {
    let (min_x, max_x, min_y, max_y) = get_boundary(map);
    let area = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize;
    area - map.len()
}

pub fn draw_map(map: &HashMap<Coordinate, Creature>) {
    let (min_x, max_x, min_y, max_y) = get_boundary(map);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut char = match map.get(&(x, y)) {
                Some(_) => '😀',
                None => '⬛',
            };
            if char != '😀' && x == 0 && y == 0 {
                char = '⬜';
            }
            print!("{}", char);
        }
        println!()
    }
    println!()
}

fn get_boundary(map: &HashMap<Coordinate, Creature>) -> Boundary {
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    for (x, y) in map.keys() {
        if *x < min_x {
            min_x = *x
        }
        if *x > max_x {
            max_x = *x
        }
        if *y < min_y {
            min_y = *y
        }
        if *y > max_y {
            max_y = *y
        }
    }
    (min_x, max_x, min_y, max_y)
}
