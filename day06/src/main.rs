use file;
use std::collections::HashMap;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut orbit_count = 0;
    let orbits = get_orbits(lines);
    for mut key in orbits.keys() {
        let mut count = 0;
        while *key != "COM" {
            key = orbits.get(key).unwrap();
            count += 1;
        }
        orbit_count += count
    }
    orbit_count
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut total_distance = 0;
    let orbits = get_orbits(lines);
    let you_parent = orbits.get("YOU").unwrap();
    let san_parent = orbits.get("SAN").unwrap();

    let mut san_distances: HashMap<&str, i32> = HashMap::new();
    let mut key = san_parent;
    let mut distance = 0;
    while *key != "COM" {
        key = orbits.get(key).unwrap();
        distance += 1;
        san_distances.insert(key, distance);
    }
    key = you_parent;
    distance = 0;
    while !san_distances.contains_key(key) {
        key = orbits.get(key).unwrap();
        distance += 1;
    }
    total_distance += distance;
    total_distance += san_distances.get(key).unwrap();

    total_distance
}

fn get_orbits(lines: &Vec<String>) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    for line in lines {
        let items: Vec<&str> = line.split(")").collect();
        map.insert(items[1], items[0]);
    }
    map
}
