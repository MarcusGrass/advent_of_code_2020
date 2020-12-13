use crate::util::{modulo, modulo64};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::Vacant;

pub fn solve_both(session: &str) {
    //let lines = crate::util::fetch_lines(13, session);
    let lines = vec![
        String::from("939"),
        String::from("7,13,x,x,59,x,31,19"),
    ];
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let schedule = to_schedule(&lines);
    let mut wait = i64::max_value();
    let mut id = 0i64;
    for bus in &schedule.buss_departures {
        let current = find_wait(&schedule.earliest_departure.clone(), bus);
        if wait > current {
            wait = current;
            id = bus.clone() as i64;
        }
    }
    println!("12.1 = {:?}", wait * id);
}

fn find_wait(departure: &i64, bus_id: &i64) -> i64 {
    bus_id - modulo64(departure.clone(), bus_id.clone())
}

fn to_schedule(lines: &Vec<String>) -> Schedule {
    let earliest_departure = lines[0].parse().unwrap();
    let mut buss_departures = Vec::new();
    for val in lines[1].split(",") {
        if val != "x" {
            buss_departures.push(val.parse().unwrap());
        }
    }
    Schedule{earliest_departure, buss_departures}
}

fn solve_second(lines: &Vec<String>) {
    let departures = to_departures(lines);
    let colls = find_collisions(&departures);
    let mutual = find_common_denominator(&colls, &departures);
    println!("{:?}", &departures);
    println!("{:?}", colls);
    println!("{:?}", mutual);
    let mut it = 1;
    loop {
        let next = next_possible(it, &departures);
        let t = next.0 - next.1;
        if modulo64(t, departures[0].id as i64) == 0 {
            if matches(t, &departures) {
                println!("12.2 = {:?}", t);
                break;
            }
        }
        it += 1;
    }
    println!("{:?}", it);

}

fn next_possible(it: i64, departures: &Vec<Departure>) -> (i64, i64) {
    let mut max_ind = 0;
    for i in 0..departures.len() {
        if departures[i].id > departures[max_ind].id {
            max_ind = i;
        }
    }
    (departures[max_ind].id as i64 * it, departures[max_ind].offset as i64)
}

fn calculate(departures: &Vec<Departure>) -> i64 {
    let mut product = 1i64;
    for departure in departures {
        product *= (departure.id - departure.offset) as i64;
    }
    product
}

fn find_collisions(departures: &Vec<Departure>) -> Vec<HashSet<usize>> {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 0..departures.len() {
        for j in 0..departures.len() {
            if i == j {
                continue;
            }
            let offset_diff = (departures[i].offset - departures[j].offset).abs();
            if modulo64(departures[i].id, offset_diff) == 0 {
                map.insert(i, j);
            }
        }
    }
    let mut groups: Vec<HashSet<usize>> = Vec::new();
    for val in &map {
        let mut group = HashSet::new();
        group.insert(val.0.clone());
        group.insert(val.1.clone());
        for val2 in &map {
            if val == val2 {
                continue;
            }
            if val2.0 == val.0 || val2.1 == val.1 || val2.0 == val.1 || val2.1 == val.0 {
                group.insert(val2.0.clone());
                group.insert(val2.1.clone());
            }
        }
        if groups.iter()
            .all(|v| {
                v.len() != group.len() || v.iter()
                    .sum::<usize>() != group.iter().sum::<usize>()
            }) {
            groups.push(group);
        }
    }
    let mut max_product = 0;
    for val in map {
        let mut product;
        if departures[val.0].offset > departures[val.1].offset {
            product = departures[val.0].id * (departures[val.1].id + departures[val.1].offset - departures[val.0].offset);
        } else {
            product = departures[val.1].id * (departures[val.0].id + departures[val.0].offset - departures[val.1].offset);
        }
        if product > max_product {
            println!("{:?} {:?} {:?}", product, departures[val.0], departures[val.1]);
            max_product = product;
        }
    }
    println!("{:?}", max_product);
    groups
}

fn find_common_denominator(groups: &Vec<HashSet<usize>>, departures: &Vec<Departure>) -> i64 {
    let mut commons = Vec::new();
    for group in groups {
        let mut product = 1i64;
        let max = &departures[find_max_offset_ind(&group.iter().map(|ind| departures[*ind].clone()).collect())];
        for ind in group {
            product *= (max.offset - departures[*ind].offset + departures[*ind].id);
        }
        commons.push(product);
    }
    *commons.iter().max().unwrap()
}

fn find_max_offset_ind(departures: &Vec<Departure>) -> usize {
    let mut max_val = 0i64;
    let mut max_ind = 0usize;
    for i in 0..departures.len() {
        if departures[i].offset > max_val {
            max_val = departures[i].offset;
            max_ind = i;
        }
    }
    max_ind
}

fn find_min_offset_ind(departures: &Vec<Departure>) -> usize {
    let mut min_val = i64::max_value();
    let mut min_ind = 0usize;
    for i in 0..departures.len() {
        if departures[i].offset < min_val {
            min_val = departures[i].offset;
            min_ind = i;
        }
    }
    min_ind
}

fn matches(t: i64, departures: &Vec<Departure>) -> bool {
    for departure in departures {
        if departure.offset != modulo64(find_wait(&t, &departure.id), departure.id) {
            return false;
        }
    }
    true
}

fn to_departures(lines: &Vec<String>) -> Vec<Departure> {
    let mut it = 0;
    let mut departures = Vec::new();
    for chr in lines[1].split(",") {
        if chr != "x" {
            departures.push(Departure{id: chr.parse().unwrap(), offset: it.clone()})
        }
        it += 1;
    }
    departures
}

#[derive(Debug)]
struct Schedule {
    earliest_departure: i64,
    buss_departures: Vec<i64>
}

#[derive(Debug, Copy, Clone)]
struct Departure {
    id: i64,
    offset: i64
}