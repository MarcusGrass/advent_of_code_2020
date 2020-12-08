use crate::solutions::aoc_eight::InstructionType::{JMP, ACC, NOP};
use std::collections::HashSet;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(8, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let instructions = to_instructions(lines);
    let mut visited = HashSet::new();
    let mut accumulation = 0;
    let mut ind: i32 = 0;
    loop {
        if !visited.insert(ind) {
            break;
        }
        let instruction = &instructions[ind as usize];
        match instruction.kind {
            ACC => {
                accumulation += instruction.value;
                ind += 1;
            },
            JMP => {
                ind += instruction.value;
            },
            NOP => ind += 1,
        }
    }
    println!("8.1 = {:?}", accumulation);

}

fn to_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        let mut splt = line.split(" ");
        let kind = match splt.next().unwrap() {
            "jmp" => JMP,
            "acc" => ACC,
            "nop" => NOP,
            a => panic!("Unrecognized value {}", a)
        };
        instructions.push(Instruction{ value: splt.next().unwrap().parse().unwrap(), kind })
    }
    instructions
}

fn solve_second(lines: &Vec<String>) {
    let instructions = to_instructions(lines);
    println!("8.2 = {:?}", brute_force(&instructions));
}

fn brute_force(instructions: &Vec<Instruction>) -> i32 {
    let mut attempts = 0;
    let mut result;
    loop {
        let instr = replace_one_jmp(&instructions, attempts);
        result = got_to_end(&instr);
        if result.is_some() {
            break;
        }
        attempts += 1;
    }
    result.unwrap()
}

fn got_to_end(instructions: &Vec<Instruction>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut accumulation = 0;
    let mut ind: i32 = 0;
    loop {
        if !visited.insert(ind) {
            break;
        }
        if ind == 625 {
            return Some(accumulation);
        }
        let instruction = &instructions[ind as usize];
        match instruction.kind {
            ACC => {
                accumulation += instruction.value;
                ind += 1;
            },
            JMP => {
                ind += instruction.value;
            },
            NOP => ind += 1,
        }
    }
    None
}


fn replace_one_jmp(instructions: &Vec<Instruction>, attempt: i32) -> Vec<Instruction> {
    let mut transformed = Vec::with_capacity(instructions.len());
    let mut seen = 0;
    for i in 0..instructions.len() {
        let instruction = &instructions[i];
        match instruction.kind {
            JMP => {
                if seen == attempt {
                    transformed.push(Instruction{ value: instruction.value, kind: InstructionType::NOP })
                } else {
                    transformed.push(instruction.clone());
                }
                seen += 1;
            },
            _ => transformed.push(instruction.clone()),
        }
    }
    transformed
}

#[derive(Debug, Clone)]
struct Instruction {
    value: i32,
    kind: InstructionType,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum  InstructionType {
    JMP,
    ACC,
    NOP
}
