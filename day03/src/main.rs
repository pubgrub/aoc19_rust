use file;
use std::cmp;

const TEST: i32 = 0;

#[derive(Debug)]
struct WireSectionNoDir {
    base: i32,
    start: i32,
    end: i32,
}

struct WireBySections {
    start_dir: i32,
    sections: [Vec<WireSectionNoDir>; 2],
}

struct WireSectionWithDir {
    dir: usize, // 0=-y, 1=x, 2=y, 3=-x
    length: i32,
}

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let wires: [WireBySections; 2] = [
        get_wire_by_section(&lines[0]),
        get_wire_by_section(&lines[1]),
    ];

    let mut min_distance = i32::MAX;
    for (x, y) in get_crossings(&wires) {
        min_distance = cmp::min(min_distance, x.abs() + y.abs());
    }
    min_distance
}

fn solve2(lines: &Vec<String>) -> i32 {
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let wires: [Vec<WireSectionWithDir>; 2] = [get_wire(&lines[0]), get_wire(&lines[1])];
    let mut min_distance = i32::MAX;
    // println!(
    //     "{:?}",
    //     get_crossings(&[
    //         get_wire_by_section(&lines[0]),
    //         get_wire_by_section(&lines[1])
    //     ])
    // );
    for (target_x, target_y) in get_crossings(&[
        get_wire_by_section(&lines[0]),
        get_wire_by_section(&lines[1]),
    ]) {
        let mut sum_distance = 0;
        'wire: for wire in &wires {
            let mut x = 0;
            let mut y = 0;
            let mut steps = 0;
            for section in wire {
                for _step in 0..section.length {
                    x += moves[section.dir].0;
                    y += moves[section.dir].1;
                    steps += 1;
                    if x == target_x && y == target_y {
                        sum_distance += steps;
                        continue 'wire;
                    }
                }
            }
        }
        min_distance = cmp::min(min_distance, sum_distance)
    }

    min_distance
}

fn get_wire_by_section(line: &String) -> WireBySections {
    let mut x_sections: Vec<WireSectionNoDir> = vec![];
    let mut y_sections: Vec<WireSectionNoDir> = vec![];
    let sections: Vec<&str> = line.split(',').collect();
    let mut x = 0;
    let mut y = 0;
    let start_dir = match &line[0..1] {
        "L" | "R" => 0,
        "D" | "U" => 1,
        _ => panic!("No direction found"),
    };
    for s in sections {
        let (dir, length) = s.split_at(1);
        let length = length.parse::<i32>().expect("no number found");
        match dir {
            "D" => {
                let wire = WireSectionNoDir {
                    base: x,
                    start: y - length,
                    end: y,
                };
                y_sections.push(wire);
                y -= length;
            }
            "U" => {
                let wire = WireSectionNoDir {
                    base: x,
                    start: y,
                    end: y + length,
                };
                y_sections.push(wire);
                y += length;
            }
            "L" => {
                let wire = WireSectionNoDir {
                    base: y,
                    start: x - length,
                    end: x,
                };
                x_sections.push(wire);
                x -= length;
            }
            "R" => {
                let wire = WireSectionNoDir {
                    base: y,
                    start: x,
                    end: x + length,
                };
                x_sections.push(wire);
                x += length;
            }
            _ => (),
        }
    }
    WireBySections {
        start_dir,
        sections: [x_sections, y_sections],
    }
}

fn get_crossings(wires: &[WireBySections; 2]) -> Vec<(i32, i32)> {
    let mut crossings: Vec<(i32, i32)> = vec![];
    for i in 0..=1 {
        for x_sec in &wires[i].sections[0] {
            for y_sec in &wires[1 - i].sections[1] {
                if y_sec.base > x_sec.start
                    && y_sec.base < x_sec.end
                    && x_sec.base > y_sec.start
                    && x_sec.base < y_sec.end
                {
                    crossings.push((y_sec.base, x_sec.base));
                }
            }
        }
    }
    crossings
}

fn get_wire(line: &String) -> Vec<WireSectionWithDir> {
    let sections: Vec<&str> = line.split(',').collect();
    let mut wire: Vec<WireSectionWithDir> = vec![];
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    for s in sections {
        let (dir_char, length) = s.split_at(1);
        let length = length.parse::<i32>().expect("no number found");
        let mut new_x = -1;
        let mut new_y = -1;
        match dir_char {
            "U" => {
                dir = 0;
                new_x = x;
                new_y = y - length;
            }
            "R" => {
                dir = 1;
                new_x = x + length;
                new_y = y;
            }
            "D" => {
                dir = 2;
                new_x = x;
                new_y = y + length;
            }
            "L" => {
                dir = 3;
                new_x = x;
                new_y = y - length;
            }
            _ => (),
        }
        wire.push(WireSectionWithDir { dir, length });
        x = new_x;
        y = new_y;
    }
    wire
}
