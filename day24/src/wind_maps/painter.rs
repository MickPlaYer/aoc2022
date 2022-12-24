use super::{TileType, WindMap, WindMaps};

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    Any,
}

pub fn draw_wind_maps(wind_maps: &WindMaps) {
    draw_wind_map(&wind_maps[0], &Direction::Right);
    draw_wind_map(&wind_maps[1], &Direction::Left);
    draw_wind_map(&wind_maps[2], &Direction::Up);
    draw_wind_map(&wind_maps[3], &Direction::Down);
}

pub fn draw_wind_map(wind_maps: &WindMap, direction: &Direction) {
    let wind_mark = match direction {
        Direction::Right => "👉",
        Direction::Left => "👈",
        Direction::Up => "👆",
        Direction::Down => "👇",
        Direction::Any => "👊",
    };
    for row in wind_maps {
        for tile_type in row {
            let mark = match tile_type {
                TileType::Wall => "🧱",
                TileType::Empty => "⬛",
                TileType::Wind => wind_mark,
            };
            print!("{}", mark);
        }
        println!();
    }
    println!();
}
