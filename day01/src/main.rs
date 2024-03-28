use std::fs::read_to_string;

fn read_file(filename:&str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }
    lines
}

fn main() {
    let lines = read_file("data/day01/input.txt");
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> i32 {
    let mut sum:i32 = 0;
    for line in lines {
        sum += line.parse::<i32>().expect("Not a number: {line}") / 3 - 2;
    }
    sum
}

fn solve2(lines:&Vec<String>) -> i32 {
    let mut sum:i32 = 0;
    for line in lines {
        let mut mass = line.parse::<i32>().expect("Not a number: {line}");
        while mass >= 9 {
            mass = mass / 3 - 2;
            sum += mass;
        }
    }
    sum
}

