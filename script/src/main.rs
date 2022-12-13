use bpaf::Bpaf;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options)]
struct Options {
    new_day: usize,
}

impl Options {
    fn get_day(&self) -> String {
        format!("day{}", self.new_day)
    }
}

fn main() {
    let options = options().run();
    add_workspace_members(&options);
    command_cargo_new(&options);
    add_shared_dependency(&options);
    create_sample_and_input(&options);
    copy_template(&options);
    println!("Done!");
}

fn add_workspace_members(options: &Options) {
    let cargo_file = "Cargo.toml";
    let insert_after = "members = [";
    let new_line = format!("    \"{}\",", options.get_day());
    insert_new_line_after(cargo_file, insert_after, &new_line);
}

fn command_cargo_new(options: &Options) {
    let output = Command::new("cargo")
        .arg("new")
        .arg("--vcs=none")
        .arg("--lib")
        .arg(&options.get_day())
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn add_shared_dependency(options: &Options) {
    let cargo_file = format!("{}/Cargo.toml", &options.get_day());
    let insert_after = "[dependencies]";
    let new_line = "shared = { path = \"../shared\" }";
    insert_new_line_after(&cargo_file, insert_after, new_line);
}

fn create_sample_and_input(options: &Options) {
    let dir_name = format!("files/{}", &options.get_day());
    fs::create_dir(&dir_name)
        .unwrap_or_else(|err| panic!("failed to create dir {}({})", &dir_name, err));
    let file_name = format!("{}/sample.txt", &dir_name);
    fs::write(&file_name, "")
        .unwrap_or_else(|err| panic!("failed to create file {}({})", &dir_name, err));
    let file_name = format!("{}/input.txt", &dir_name);
    fs::write(&file_name, "")
        .unwrap_or_else(|err| panic!("failed to create file {}({})", &dir_name, err));
}

fn copy_template(options: &Options) {
    let template_path = "files/template/lib.rs";
    let target_path = format!("{}/src/lib.rs", &options.get_day());
    fs::copy(template_path, target_path).expect("Can not copy files/template/lib.rs !");
}

fn insert_new_line_after(cargo_file: &str, insert_after: &str, new_line: &str) {
    let cargo =
        fs::read_to_string(cargo_file).unwrap_or_else(|_| panic!("Can not read {}!", cargo_file));
    let mut lines = cargo.lines().collect::<Vec<&str>>();
    let index = lines
        .iter()
        .position(|&r| r == insert_after)
        .unwrap_or_else(|| panic!("Can not find {}!", insert_after));
    lines.insert(index + 1, new_line);
    let mut result = lines.join("\n");
    result.push('\n');
    fs::write(cargo_file, &result).unwrap_or_else(|_| panic!("Can not write {}!", cargo_file));
}
