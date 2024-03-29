use file;

const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    println!("{day}");
    let lines = file::read_file(day, TEST);
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

