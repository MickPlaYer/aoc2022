use crate::wind_maps::{get_outer_size, TileType, WindMaps};
use std::{collections::HashMap, fmt::Debug};

type Point = (usize, usize);
type SizeInfo = (usize, usize);

pub struct State {
    wind_index: usize,
    point: Point,
    distance: usize,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State {} ({}, {}) => {}",
            &self.wind_index, &self.point.0, &self.point.1, &self.distance
        )
    }
}

impl State {
    pub fn new(wind_index: usize, point: Point) -> Self {
        Self {
            wind_index,
            point,
            distance: usize::MAX,
        }
    }

    pub fn set_distance(&mut self, distance: usize) {
        self.distance = distance;
    }

    pub fn get_distance(&self) -> usize {
        self.distance
    }

    fn hash(&self) -> (usize, Point) {
        (self.wind_index, self.point)
    }
}

pub fn find_the_shortest_path(all_wind_status: &WindMaps, from: State, to: Point) -> Option<State> {
    let size_info = get_outer_size(&all_wind_status);
    let start_state = from;
    let end_point = to;
    let mut visited = HashMap::new();
    let mut search = vec![start_state];
    loop {
        if search.is_empty() {
            break None;
        }
        search.sort_by(|a, b| b.distance.cmp(&a.distance));
        let current_state = search.pop().unwrap();
        if visited.contains_key(&current_state.hash()) {
            continue;
        }
        let next_index = (current_state.wind_index + 1) % all_wind_status.len();
        let next_wind_map = &all_wind_status[next_index];
        let neighbors = get_neighbors(&current_state.point, &size_info);
        for (x, y) in neighbors {
            let mut new_state = State::new(next_index, (x, y));
            let tile_type = &next_wind_map[y][x];
            if let TileType::Empty = tile_type {
                update_distance(&current_state, &mut new_state);
                search.push(new_state);
            }
        }
        if current_state.point == end_point {
            break Some(current_state);
        }
        visited.insert(current_state.hash(), current_state);
    }
}

fn update_distance(current_state: &State, new_state: &mut State) {
    let current_distance = current_state.distance;
    let old_distance = new_state.distance;
    let new_distance = current_distance + 1;
    if new_distance < old_distance {
        new_state.distance = new_distance;
    }
}

fn get_neighbors(point: &Point, size_info: &SizeInfo) -> Vec<Point> {
    let (width, height) = size_info;
    let (x, y) = point;
    let mut neighbors = vec![(*x, *y)];
    if *x > 0 {
        neighbors.push((x - 1, *y));
    }
    if *x < width - 1 {
        neighbors.push((x + 1, *y));
    }
    if *y > 0 {
        neighbors.push((*x, y - 1));
    }
    if *y < height - 1 {
        neighbors.push((*x, y + 1));
    }
    neighbors
}
