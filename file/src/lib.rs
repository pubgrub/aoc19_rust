use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
pub fn read_file(day: &str, test: i32) -> Vec<String> {
    println!("{:?}", current_dir());
    let filename: String = match test {
        0 => "input.txt".to_string(),
        other => "test".to_string() + &(other.to_string()) + ".txt",
    };
    // let working_path = current_dir().unwrap();
    // let working_path_parent = working_path.parent().unwrap();

    // let mut path = working_path_parent.to_path_buf();
    let mut path = PathBuf::new();

    // hard coded due to different working dir when running or debugging
    path.push("C:\\Users\\fox\\dev\\aoc19_rust\\data");
    path.push(day.to_string());
    path.push(filename);

    let mut lines = Vec::new();
    println!("{:?}", path);
    for line in read_to_string(path).unwrap().lines() {
        lines.push(line.to_string());
    }
    lines
}
