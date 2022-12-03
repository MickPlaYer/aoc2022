use std::{env, fs};

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No file path found in args!");
    let file_path = file_path.as_str();
    fs::read_to_string(file_path.to_owned())
        .expect(format!("Fail to read file {}", file_path).as_str())
}
