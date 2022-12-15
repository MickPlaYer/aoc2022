use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
};

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No file path found in args!");
    let file_path = file_path.as_str();
    read_file_internal(file_path)
}

pub fn read_sample(day_num: usize) -> String {
    let sample_path = file_path(day_num, "sample.txt");
    let content = read_file_internal(&sample_path);
    if content.is_empty() {
        panic!("sample.txt is empty!")
    }
    content
}

pub fn read_input(day_num: usize) -> String {
    let input_path = file_path(day_num, "input.txt");
    let content = read_file_internal(&input_path);
    if content.is_empty() {
        panic!("input.txt is empty!")
    }
    content
}

fn file_path(day_num: usize, file_name: &str) -> String {
    let root = env::current_dir().unwrap();
    let root = root.parent().unwrap();
    let binding = root
        .join("files")
        .join(format!("day{}", day_num))
        .join(file_name);
    String::from(binding.to_str().unwrap())
}

fn read_file_internal(file_path: &str) -> String {
    fs::read_to_string(file_path.to_owned())
        .expect(format!("Fail to read file {}", file_path).as_str())
}

pub fn log(text: &str) {
    let root = env::current_dir().unwrap();
    let root = root.parent().unwrap();
    let binding = root.join("log").join("output.log");
    let mut file = OpenOptions::new().append(true).open(binding).unwrap();
    write!(file, "{}\n", text).unwrap();
}
