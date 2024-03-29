use std::fs::read_to_string;

pub fn read_file(day:&str, test:i32) -> Vec<String> {
    let filename:String = match test {
        0 => "input.txt".to_string(),
        other => "test".to_string() + &(other.to_string()) + ".txt",
    };
    let path = "data/".to_string() + day + "/" +  filename.as_str();
    let mut lines = Vec::new();
    println!("{path}");
    for line in read_to_string(path).unwrap().lines() {
        lines.push(line.to_string());
    }
    lines
}
