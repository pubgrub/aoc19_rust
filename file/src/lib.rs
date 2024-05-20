use std::fs::read_to_string;
use std::path::PathBuf;
//use std::env::current_dir;
pub fn read_file(day: &str, test: i32) -> Vec<String> {
    //println!("{:?}", current_dir());
    let filename: String = match test {
        0 => "input.txt".to_string(),
        other => "test".to_string() + &(other.to_string()) + ".txt",
    };
    let mut path = PathBuf::new();

    println!("running on OS: {}", std::env::consts::OS);
    // hard coded due to different working dir when running or debugging
    let os_path = match std::env::consts::OS {
        "windows" => "C:\\Users\\fox\\dev\\aoc19_rust\\data",
        "linux" => "/home/fox/aoc19_rust/data",
        _ => "",
    };
    path.push(os_path);
    path.push(day.to_string());
    path.push(filename);

    let mut lines = Vec::new();
    //println!("{:?}", path);
    for line in read_to_string(path).unwrap().lines() {
        lines.push(line.to_string());
    }
    lines
}
