use std::collections::HashMap;
static LENGTH: usize = 36usize;
pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(14, session);
    solve_first(&lines);
    solve_second(&lines);

}

fn solve_first(lines: &Vec<String>) {
    let inputs = to_input(lines);
    let mut sum_map = HashMap::new();
    for input in &inputs {
        for instr in &input.instructions {
            let pass_val = pass(&input.mask, instr.value);
            sum_map.insert(instr.mem_loc, pass_val);
        }
    }
    let total: usize = sum_map.values()
        .sum();
    println!("14.1 = {:?}", total);
}

fn solve_second(lines: &Vec<String>) {
    let inputs = to_second_input(lines);
    let mut map = HashMap::new();
    for input in &inputs {
        for instruction in &input.instructions {
            let addrs = get_addresses(&input.mask, instruction.mem_loc);
            for addr in addrs {
                map.insert(addr, instruction.value);
            }
        }
    }
    let total: usize = map.values()
        .sum();
    println!("14.2 = {:?}", total);
}

fn get_addresses(mask: &Mask, value: usize) -> Vec<usize> {
    let mut num_masks = 0;
    for val in &mask.map {
        if val.1 == &2 {
            num_masks += 1;
        }
    }
    let perms = get_permutations(num_masks);
    let mut addrs = Vec::new();
    for perm in perms {
        let mut map = mask.map.clone();
        let mut it = 0;
        for i in 0..LENGTH {
            match map.get(&i) {
                Some(o) => {
                    if o == &2 {
                        map.insert(i, perm[it]);
                        it +=  1;
                    }
                }
                None => ()
            }
        }
        addrs.push(pass(&Mask{map}, value));
    }
    addrs
}

fn get_permutations(num: usize) -> Vec<Vec<usize>> {
    let mut combos = Vec::new();
    for i in 0..u64::pow(2, num as u32) {
        let mut s = Vec::new();
        for j in 0..num {
            if (i & (1 << j)) == 0 {
                s.push(0)
            } else {
                s.push(1)
            }
        }
        combos.push(s);
    }
    combos
}

fn pass(mask: &Mask, value: usize) -> usize {
    let mut as_str = to_padded_byte_vec(value);
    for val in &mask.map {
        as_str[*val.0] = *val.1;
    }
    let mut total = 0;
    for i in 0..as_str.len() {
        total += as_str[i] * usize::pow(2, i as u32)
    }
    total
}


fn to_padded_byte_vec(val: usize) -> Vec<usize> {
    let as_str = format!("{:0>36}", format!("{:b}", val));
    let mut v = Vec::new();
    for char in as_str.chars().rev() {
        v.push(char.to_digit(10).unwrap() as usize)
    }
    v
}

fn to_input(lines: &Vec<String>) -> Vec<Input> {
    let mut input = Vec::new();
    let mut mask = Mask{map: HashMap::new()};
    let mut instructions = Vec::new();
    for line in lines {
        if line.starts_with("mask") {
            if !instructions.is_empty() {
                input.push(Input{mask, instructions: instructions.clone()});
                instructions = Vec::new();
            }
            mask = Mask{map: to_map(line)};
        } else {
            instructions.push(to_instruction(line))
        }
    }
    input.push(Input{mask, instructions: instructions.clone()});
    input
}


fn to_second_input(lines: &Vec<String>) -> Vec<Input> {
    let mut input = Vec::new();
    let mut mask = Mask{map: HashMap::new()};
    let mut instructions = Vec::new();
    for line in lines {
        if line.starts_with("mask") {
            if !instructions.is_empty() {
                input.push(Input{mask, instructions: instructions.clone()});
                instructions = Vec::new();
            }
            mask = Mask{map: to_x_map(line)};
        } else {
            instructions.push(to_instruction(line))
        }
    }
    input.push(Input{mask, instructions: instructions.clone()});
    input
}

fn to_map(str: &String) -> HashMap<usize, usize> {
    let mut it = 1;
    let mut map = HashMap::new();
    let mut splt = str.split("=");
    splt.next();
    for chr in splt.next().unwrap().chars() {
        if chr == ' ' {
            continue;
        }
        if chr == '1' {
            map.insert(LENGTH - it, 1);
        } else if chr == '0' {
            map.insert(LENGTH - it, 0);
        }
        it += 1;
    }
    map
}


fn to_x_map(str: &String) -> HashMap<usize, usize> {
    let mut it = 1;
    let mut map = HashMap::new();
    let mut splt = str.split("=");
    splt.next();
    for chr in splt.next().unwrap().chars() {
        if chr == ' ' {
            continue;
        }
        if chr == '1' {
            map.insert(LENGTH - it, 1);
        } else if chr == 'X' {
            map.insert(LENGTH - it, 2);
        }
        it += 1;
    }
    map
}

fn to_instruction(str: &String) -> Instruction {
    let mut splt = str.split("=");
    let mut chars = String::new();
    let mut started = false;
    for chr in splt.next().unwrap().chars() {
        if chr == '[' {
            started = true;
        } else if chr == ']' {
            break
        } else if started {
            chars = format!("{}{}", chars, chr);
        }
    }
    let mem_loc = chars.parse().unwrap();
    let value = splt.next().unwrap().trim().parse().unwrap();
    Instruction{mem_loc, value}
}

#[derive(Debug, Clone)]
struct Input {
    mask: Mask,
    instructions: Vec<Instruction>
}


#[derive(Debug, Clone)]
struct Mask {
    map: HashMap<usize, usize>
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    mem_loc: usize,
    value: usize,
}
