use file;
use std::cmp;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let seeds: Vec<i32> = vec![];
    let program = get_program(&lines[0]);
    let result: i32 = chain_programs(program, 0, seeds);
    result
}

#[derive(Debug)]
struct Computer {
    program: Vec<i32>,
    first_run:bool,
    primer:i32,
    output:i32,
}

enum ReturnCode {
    Value(i32),
    Halt(),
}

fn solve2(lines: &Vec<String>) -> i32 {
    let program = get_program(&lines[0]);
    let mut combi:Vec<i32> = vec![];
    let mut combi_list:Vec<Vec<i32>> = vec![];
    let combinations = get_combinations(combi, combi_list);
    //println!("{:?}", combinations);

    for com in combinations {
        let mut computers:Vec<Computer> = vec![];
        for i in 0..5 {
            let mut c = Computer {
                program: program.clone(),
                first_run: true,
                primer:com[i],
                output:0,
            };
            computers.push(c);
        }
        //println!("{:?}",computers);
        let mut comp_idx = 0;
        loop {
            let input = computers[(comp_idx - 1) % 5].output;
            if comp_idx == 5 {
                comp_idx = 0;
            }
            let comp: &mut Computer = &mut computers[comp_idx];
            let mut para:Vec<i32> = vec![]; 
            if comp.first_run == true {
                para.push(comp.primer);
            }
            para.push(input);
            comp.output = run_program(&mut comp.program, para);
        }

    }

    let mut result: i32 = 0;
    result
}

fn get_combinations(mut combi: Vec<i32>, mut combi_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if combi.len() == 5 {
        combi_list.push(combi);
        return combi_list;
    }
    for i in 5..=9 {
        if combi.contains(&i) {
            continue;
        }
        let mut new_combi = combi.clone();
        new_combi.push(i);
        combi_list = get_combinations(new_combi,combi_list);
    
    }
    return combi_list;
}


fn chain_programs(program: Vec<i32>, result_in: i32, seeds: Vec<i32>) -> i32 {
    let mut result = result_in;
    for i in 0..=4 {
        if seeds.contains(&i) {
            continue;
        }
        let mut my_program = program.clone();
        let my_result = run_program(&mut my_program, [i, result_in].to_vec());
        let mut new_seeds = seeds.clone();
        new_seeds.push(i);
        result = i32::max(
            result,
            chain_programs(program.clone(), my_result, new_seeds),
        );
    }
    result
}

fn get_program(line: &str) -> Vec<i32> {
    line.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn run_program(program: &mut Vec<i32>, input: Vec<i32>) -> ReturnCode {
    let mut next_input = 0;
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
            99 => return ReturnCode::Halt(),
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
                program[in_out_addr] = input[next_input];
                if next_input < input.len() - 1 {
                    next_input += 1;
                }
                instr_ptr += 2;
            }
            4 => {
                return ReturnCode::Value(program[in_out_addr]);
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
    panic!("run_program did not return wirh output value");
}
