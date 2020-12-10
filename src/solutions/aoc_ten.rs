use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(10, session);
    let mut numbers: Vec<i64> = lines.iter()
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(*numbers.iter().max().unwrap() + 3);
    solve_first(&numbers);
    solve_second(&numbers);
}

fn solve_first(numbers: &Vec<i64>) {
    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff == 1 {
            one_jolt_diff += 1;
        } else if diff == 3 {
            three_jolt_diff += 1;
        }
    }
    println!("10.1 = {:?}", one_jolt_diff * three_jolt_diff);
}

fn solve_second(numbers: &Vec<i64>) {
    println!("10.2 = {:?}", traverse(numbers));
}

fn traverse(numbers: &Vec<i64>) -> i64 {
    let forward_refs = construct_map(numbers);
    let backward_refs = construct_dest_map(forward_refs);
    let mut cumsum: HashMap<usize, i64> = HashMap::new();
    cumsum.insert(0, 1);
    for i in 1..numbers.len() {
        cumsum.insert(i, accumulate(backward_refs.get(&i).unwrap(), &cumsum));
    }
    *cumsum.get(&(numbers.len() - 1)).unwrap()
}

fn accumulate(vec: &Vec<usize>, cumsum: &HashMap<usize, i64>) -> i64 {
    let mut sum = 0;
    for val in vec {
        sum += cumsum.get(val).unwrap();
    }
    sum
}

fn construct_dest_map(mut numbers: HashMap<usize, Vec<usize>>) -> HashMap<usize, Vec<usize>>{
    let mut dest: HashMap<usize, Vec<usize>> = HashMap::new();
    for entry in numbers.iter_mut() {
        for val in entry.1 {
            match dest.entry(*val) {
                Entry::Occupied(mut o) => o.get_mut().push(*entry.0),
                Entry::Vacant(v) => {
                    let mut vec = Vec::new();
                    vec.push(*entry.0);
                    v.insert(vec);
                }
            }
        }
    }
    dest
}

fn construct_map(numbers: &Vec<i64>) -> HashMap<usize, Vec<usize>> {
    let mut options: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[j] - numbers[i] <= 3 {
                match options.entry(i) {
                    Entry::Occupied(mut o) => o.get_mut().push(j),
                    Entry::Vacant(v) => {
                        let mut vec = Vec::new();
                        vec.push(j);
                        v.insert(vec);
                    }
                }
            }
        }
    }
    options
}

fn construct_path(numbers: &Vec<i64>) {
    let mut options: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[j] - numbers[i] <= 3 {
                match options.entry(i) {
                    Entry::Occupied(mut o) => o.get_mut().push(j),
                    Entry::Vacant(v) => {
                        let mut vec = Vec::new();
                        vec.push(j);
                        v.insert(vec);
                    }
                }
            }
        }
    }
    println!("{:?}", recurse_count(0, &options) + 1);
}

fn recurse_count(start: usize, map: &HashMap<usize, Vec<usize>>) -> i64 {
    let options = map.get(&start);
    if options.is_none() {
        return 0;
    } else {
        let vec = options.unwrap();
        let mut sum = vec.len() as i64 - 1;
        for val in vec {
            sum += recurse_count(*val, map)
        }
        sum as i64
    }
}


fn recurse_search(start: usize, numbers: &Vec<i64>) -> i64 {
    if start + 1 == numbers.len() - 1 {
        return 1;
    } else {
        let mut sum = 0;
        for i in start + 1..numbers.len() {
            if numbers[i] - numbers[start] <= 3 {
                sum += recurse_search(i, numbers);
            } else {
                break;
            }
        }
        sum
    }
}
