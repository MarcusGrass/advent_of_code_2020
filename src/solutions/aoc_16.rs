use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(16, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let ranges = get_valid_ranges(lines);
    let tickets = get_tickets(lines);
    let mut sum_err = 0;
    for ticket in &tickets {
        sum_err += in_range(ticket, &ranges);
    }
    println!("16.1 = {:?}", sum_err);
}

fn get_my_ticket(lines: &Vec<String>) -> Ticket {
    for i in 0..lines.len() {
        if lines[i].starts_with("your") {
            return to_ticket(&lines[i+1]);
        }
    }
    panic!("Could not find ticket");
}

fn in_range(ticket: &Ticket, ranges: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut sum_err = 0;
    for val in &ticket.values {
        let mut hit = false;
        for range in ranges {
            if (val >= &range.0.0 && val <= &range.0.1) || (val >= &range.1.0 && val <= &range.1.1) {
                hit = true;
                break;
            }
        }
        if !hit {
            sum_err += val;
        }

    }
    sum_err
}

fn solve_second(lines: &Vec<String>) {
    let ranges = get_valid_ranges(lines);
    let my_ticket = get_my_ticket(lines);
    let valid_tickets: Vec<Ticket> = get_tickets(lines).iter()
        .filter(|t| in_range(t, &ranges) == 0)
        .map(|t| t.to_owned())
        .collect();
    let reduced = tick_ind_of_range_ind(&ranges, &valid_tickets);
    let mut reverse = HashMap::new();
    for entry in reduced {
        reverse.insert(entry.1, entry.0);
    }
    let mut prod = 1i64;
    for i in 0..6 {
        prod *= my_ticket.values[*reverse.get(&i).unwrap()] as i64;
    }
    println!("16.2 = {:?}", prod)
}

fn tick_ind_of_range_ind(ranges: &Vec<((i32, i32), (i32, i32))>, tickets: &Vec<Ticket>) -> HashMap<usize, usize> {
    let mut sum_map = HashMap::new();
    for ticket in tickets {
        for i in 0..ticket.values.len() {
            let val = ticket.values[i];
            for j in 0..ranges.len() {
                if (val >= ranges[j].0.0 && val <= ranges[j].0.1) || (val >= ranges[j].1.0 && val <= ranges[j].1.1) {
                    match sum_map.entry(i) {
                        Entry::Vacant(v) => {
                            let mut map = HashMap::new();
                            map.insert(j, 1);
                            v.insert(map);
                            ()
                        },
                        Entry::Occupied(mut o) => {
                            match o.get_mut().entry(j) {
                                Entry::Vacant(v) => {
                                    v.insert(1);
                                    ()
                                },
                                Entry::Occupied(mut o) => {
                                    o.insert(o.get() + 1);
                                    ()
                                }
                            };
                            ()
                        }
                    }
                }
            }
        }
    }
    reduce(&sum_map, tickets.len())
}

fn reduce(sum_map: &HashMap<usize, HashMap<usize, usize>>, categories: usize) -> HashMap<usize, usize> {
    let mut reduced = HashMap::new();
    for entry in sum_map {
        for sub in entry.1 {
            if sub.1 == &categories {
                match reduced.entry(entry.0) {
                    Entry::Vacant(v) => {
                        let mut map = HashMap::new();
                        map.insert(sub.0, sub.1);
                        v.insert(map);
                        ()
                    },
                    Entry::Occupied(mut o) => {
                        o.get_mut().insert(sub.0, sub.1);
                        ()
                    }
                }
            }
        }
    }
    let mut passed: HashSet<usize> = HashSet::new();
    loop {
        let mut work_done = false;
        for entry in reduced.clone() {
            if entry.1.len() == 1 {
                let val = entry.1.iter().next().unwrap();
                if passed.contains(val.0.clone()) {
                    continue;
                } else {
                    for entry_other in reduced.iter_mut() {
                        if entry_other.0 == &entry.0 {
                            continue;
                        }
                        entry_other.1.remove(val.0.clone());
                        work_done = true;
                    }
                    passed.insert(*val.0.clone());
                }
            }
        }
        if !work_done {
            break;
        }
    }
    let mut excl: HashMap<usize, usize> = HashMap::new();
    for val in &reduced {
        excl.insert(**val.0, **val.1.iter().next().unwrap().0);
    }
    excl
}

fn get_tickets(lines: &Vec<String>) -> Vec<Ticket> {
    let mut started = false;
    let mut tickets = Vec::new();
    for line in lines {
        if line.starts_with("nearby") {
            started = true;
            continue;
        }
        if started {
            tickets.push(to_ticket(line));
        }
    }
    tickets
}

fn to_ticket(line: &String) -> Ticket {
    let mut values = Vec::new();
    for num in line.split(",") {
        values.push(num.parse().unwrap());
    }
    Ticket{values}
}

fn get_valid_ranges(lines: &Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    let mut relevant = Vec::new();
    for line in lines {
        if line != "your ticket:" {
            relevant.push(line.clone());
        } else {
            break;
        }
    }
    let mut ranges = Vec::new();
    for line in relevant {
        let mut splt = line.split(":");
        splt.next();
        let mut range_split = splt.next().unwrap().trim().split("or");
        let rn1 = to_range(range_split.next().unwrap().trim());
        let rn2 = to_range(range_split.next().unwrap().trim());
        ranges.push((rn1, rn2))
    }
    ranges
}

fn to_range(str: &str) -> (i32, i32) {
    let mut splt = str.split("-");
    let first = splt.next().unwrap();
    let second = splt.next().unwrap();
    (first.parse().unwrap(), second.parse().unwrap())
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<i32>
}
