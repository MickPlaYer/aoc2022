mod painter;
mod parser;

pub use painter::{draw_wind_map, draw_wind_maps};
pub use parser::parse_wind_maps;

#[derive(Clone, Debug)]
pub enum TileType {
    Wall,
    Empty,
    Wind,
}

type WindMap = Vec<Vec<TileType>>;
pub type WindMaps = Vec<WindMap>;

pub fn produce_all_wind_status(mut wind_maps: WindMaps) -> WindMaps {
    let (width, height) = get_inner_size(&wind_maps);
    let mut horizontal_wind_statuses = Vec::new();
    for _ in 0..width {
        let wind_map = merge_wind_map(&wind_maps[0], &wind_maps[1]);
        horizontal_wind_statuses.push(wind_map);
        for row in wind_maps[0].iter_mut().skip(1).take(height) {
            row[1..=width].rotate_right(1);
        }
        for row in wind_maps[1].iter_mut().skip(1).take(height) {
            row[1..=width].rotate_left(1);
        }
    }
    let mut vertical_wind_statuses = Vec::new();
    for _ in 0..height {
        let wind_map = merge_wind_map(&wind_maps[2], &wind_maps[3]);
        vertical_wind_statuses.push(wind_map);
        wind_maps[2][1..=height].rotate_left(1);
        wind_maps[3][1..=height].rotate_right(1);
    }
    let mut all_wind_statuses = Vec::new();
    let repeat_length = shared::lcm(horizontal_wind_statuses.len(), vertical_wind_statuses.len());
    let mut horizontal_wind_statuses = horizontal_wind_statuses.iter().cycle();
    let mut vertical_wind_statuses = vertical_wind_statuses.iter().cycle();
    for _ in 0..repeat_length {
        let horizontal_wind_status = horizontal_wind_statuses.next().unwrap();
        let vertical_wind_status = vertical_wind_statuses.next().unwrap();
        let wind_map = merge_wind_map(horizontal_wind_status, vertical_wind_status);
        all_wind_statuses.push(wind_map);
    }
    all_wind_statuses
}

fn merge_wind_map(wind_map1: &WindMap, wind_map2: &WindMap) -> WindMap {
    let mut new_wind_map = Vec::new();
    for (y, row) in wind_map1.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, tile_type1) in row.iter().enumerate() {
            let tile_type2 = &wind_map2[y][x];
            match (tile_type1, tile_type2) {
                (TileType::Wall, TileType::Wall) => new_row.push(TileType::Wall),
                (TileType::Empty, TileType::Empty) => new_row.push(TileType::Empty),
                (TileType::Empty, TileType::Wind)
                | (TileType::Wind, TileType::Empty)
                | (TileType::Wind, TileType::Wind) => new_row.push(TileType::Wind),
                _ => panic!("Impossible combination!"),
            }
        }
        new_wind_map.push(new_row);
    }
    new_wind_map
}

fn get_inner_size(wind_maps: &WindMaps) -> (usize, usize) {
    let height = wind_maps[0].len() - 2;
    let width = wind_maps[0][0].len() - 2;
    (width, height)
}

pub fn get_outer_size(wind_maps: &WindMaps) -> (usize, usize) {
    let height = wind_maps[0].len();
    let width = wind_maps[0][0].len();
    (width, height)
}
