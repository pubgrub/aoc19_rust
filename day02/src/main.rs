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
    let mut opcodes = get_program(lines[0].as_str());
    println!("{:?}",opcodes);
    if TEST == 0 {
        opcodes[1] = 12;
        opcodes[2] = 2;
    }
    run_program(opcodes) as i32
}

fn solve2(lines:&Vec<String>) -> usize {
    let program_orig = get_program(lines[0].as_str());
    let wanted:usize = lines[1].parse::<usize>().expect("did not find a number");
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program_orig.clone();
            program[1] = noun;
            program[2] = verb;
            if run_program(program) == wanted {
                return noun * 100 + verb;
            }
        }
    }
    0
}

fn get_program(line:&str) -> Vec<usize> {
    line
        .split(',')
        .filter_map(|s| s.parse().ok())  
        .collect()
    }

fn run_program(mut program:Vec<usize>) -> usize {
    let mut pointer = 0;
    loop {
        match program[pointer] {
            99 => break,
            1 => {
                let dest = program[pointer + 3];
                program[dest] = program[program[pointer + 1]] + program[program[pointer + 2]];
                pointer += 4;
            }
            2 => {
                let dest = program[pointer + 3];
                program[dest] = program[program[pointer + 1]] * program[program[pointer + 2]];
                pointer += 4;
            }
            _ => (),
        };
    }
    program[0]
}