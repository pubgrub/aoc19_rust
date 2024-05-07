use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    run_program(get_program(&lines[0]), 1i32)
}

fn solve2(lines: &Vec<String>) -> i32 {
    run_program(get_program(&lines[0]), 5i32)
}

fn get_program(line: &str) -> Vec<i32> {
    line.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn run_program(mut program: Vec<i32>, input: i32) -> i32 {
    let mut instr_ptr: usize = 0;
    let mut instr: i32;
    let mut opcode: i32;
    let mut para1_mode: i32;
    let mut para2_mode: i32;
    let mut para1_val: i32 = 0;
    let mut para2_val: i32 = 0;
    let mut output_addr: usize;
    let mut out_val: i32;
    let mut output: i32 = 0;
    let mut in_out_addr: usize = 0;
    loop {
        instr = program[instr_ptr];
        opcode = instr % 100;

        match opcode {
            1 | 2 | 5 | 6 | 7 | 8 => {
                para1_mode = instr / 100 % 10;
                para1_val = match para1_mode {
                    0 => {
                        let in1_addr: usize = program[instr_ptr + 1].try_into().unwrap();
                        program[in1_addr]
                    }
                    1 => program[instr_ptr + 1],
                    _ => panic!("in1_mode is other than 0 or 1: {instr}"),
                };
                para2_mode = instr / 1000 % 10;
                para2_val = match para2_mode {
                    0 => {
                        let in2_addr: usize = program[instr_ptr + 2].try_into().unwrap();
                        program[in2_addr]
                    }
                    1 => program[instr_ptr + 2],
                    _ => panic!("in2_mode is other than 0 or 1"),
                };
            }
            3 | 4 => {
                in_out_addr = program[instr_ptr + 1].try_into().unwrap();
            }
            _ => (),
        }

        match opcode {
            99 => break,
            1 => {
                out_val = para1_val + para2_val;
                output_addr = program[instr_ptr + 3].try_into().unwrap();
                program[output_addr] = out_val;
                instr_ptr += 4;
            }
            2 => {
                out_val = para1_val * para2_val;
                output_addr = program[instr_ptr + 3].try_into().unwrap();
                program[output_addr] = out_val;
                instr_ptr += 4;
            }
            3 => {
                program[in_out_addr] = input;
                instr_ptr += 2;
            }
            4 => {
                output = program[in_out_addr];
                instr_ptr += 2;
            }
            5 => {
                instr_ptr = if para1_val != 0 {
                    para2_val.try_into().unwrap()
                } else {
                    instr_ptr + 3
                }
            }
            6 => {
                instr_ptr = if para1_val == 0 {
                    para2_val.try_into().unwrap()
                } else {
                    instr_ptr + 3
                }
            }
            7 => {
                output_addr = program[instr_ptr + 3].try_into().unwrap();
                program[output_addr] = if para1_val < para2_val { 1 } else { 0 };
                instr_ptr += 4;
            }
            8 => {
                output_addr = program[instr_ptr + 3].try_into().unwrap();
                program[output_addr] = if para1_val == para2_val { 1 } else { 0 };
                instr_ptr += 4;
            }

            _ => (),
        };
    }
    output
}
