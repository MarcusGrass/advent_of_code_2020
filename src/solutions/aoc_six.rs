use std::collections::{HashSet, HashMap, hash_map::Entry};

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(6, session);
    let grouped = to_group_responses(&lines);
    solve_first(&grouped);
    solve_second(&grouped);
}

fn to_group_responses(lines: &Vec<String>) -> Vec<Vec<&String>> {
    let mut responses: Vec<Vec<&String>> = Vec::new();
    let mut group_vec = Vec::new();
    for line in lines {
        if line != "" {
            group_vec.push(line);
        } else {
            responses.push(group_vec.clone());
            group_vec = Vec::new();
        }
    }
    responses
}

fn solve_first(groups: &Vec<Vec<&String>>) {
    let mut num_unique = 0;
    for group in groups {
        let mut unique_chars = HashSet::new();
        for response in group {
            for chr in response.chars() {
                unique_chars.insert(chr);
            }
        }
        num_unique += unique_chars.len();
    }
    println!("6.1 = {}", num_unique);
}

fn solve_second(groups: &Vec<Vec<&String>>) {
    let mut num_unanimous = 0;
    for group in groups {
        let mut map: HashMap<char, i32> = HashMap::new();
        for response in group {
            for chr in response.chars() {
                match map.entry(chr) {
                    Entry::Occupied(mut o) => o.insert(o.get() + 1),
                    Entry::Vacant(v) => *v.insert(1)
                };
            }
        }
        for entry in map {
            if entry.1 as usize == group.len() {
                num_unanimous += 1;
            }
        }
    }
    println!("6.2 = {}", num_unanimous);
}
