use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let range: Vec<&str> = lines[0].split("-").collect();
    let start: i32 = range[0].parse().unwrap();
    let end = range[1].parse().unwrap();
    let mut solutions = 0;
    let mut last_digit;
    let mut pair_rule_satisfied;

    'n: for n in start..=end {
        let mut number = n;
        last_digit = -1;
        pair_rule_satisfied = false;
        let mut div = 100000;
        for _pos in 0..6 {
            let digit = number / div;
            if digit < last_digit {
                continue 'n;
            }
            if digit == last_digit {
                pair_rule_satisfied = true;
            }
            last_digit = digit;
            number -= div * digit;
            div /= 10;
        }
        if pair_rule_satisfied == true {
            solutions += 1;
        }
    }
    solutions
}

fn solve2(lines: &Vec<String>) -> i32 {
    let range: Vec<&str> = lines[0].split("-").collect();
    let start: i32 = range[0].parse().unwrap();
    let end = range[1].parse().unwrap();
    let mut solutions = 0;
    let mut last_digit;
    let mut pair_rule_satisfied;
    let mut group_length: i32;

    'n: for n in start..=end {
        let mut number = n;
        last_digit = -1;
        pair_rule_satisfied = false;
        group_length = 0;
        let mut div = 100000;

        for _pos in 0..6 {
            let digit = number / div;
            if digit < last_digit {
                continue 'n;
            }
            if digit == last_digit {
                group_length += 1;
            } else {
                if group_length == 2 {
                    pair_rule_satisfied = true;
                }
                group_length = 1;
            }
            last_digit = digit;
            number -= div * digit;
            div /= 10;
        }
        if pair_rule_satisfied == true || group_length == 2 {
            solutions += 1;
        }
    }
    solutions
}
