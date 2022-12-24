use super::{TileType, WindMaps};

pub fn parse_wind_maps(content: String) -> WindMaps {
    let mut wind_maps = vec![Vec::new(); 4];
    for line in content.lines() {
        let mut wind_rows = vec![Vec::new(); 4];
        for char in line.chars() {
            match char {
                '#' => {
                    for i in 0..4 {
                        wind_rows[i].push(TileType::Wall);
                    }
                }
                '.' => {
                    for i in 0..4 {
                        wind_rows[i].push(TileType::Empty);
                    }
                }
                '>' => {
                    wind_rows[0].push(TileType::Wind);
                    for i in [1, 2, 3] {
                        wind_rows[i].push(TileType::Empty);
                    }
                }
                '<' => {
                    wind_rows[1].push(TileType::Wind);
                    for i in [0, 2, 3] {
                        wind_rows[i].push(TileType::Empty);
                    }
                }
                '^' => {
                    wind_rows[2].push(TileType::Wind);
                    for i in [0, 1, 3] {
                        wind_rows[i].push(TileType::Empty);
                    }
                }
                'v' => {
                    wind_rows[3].push(TileType::Wind);
                    for i in [0, 1, 2] {
                        wind_rows[i].push(TileType::Empty);
                    }
                }
                _ => (),
            }
        }
        for (i, wind_row) in wind_rows.into_iter().enumerate() {
            wind_maps[i].push(wind_row);
        }
    }
    wind_maps
}
