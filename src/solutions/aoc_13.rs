use crate::util::{modulo, modulo64};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::Vacant;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(13, session);
    /*
    let lines = vec![
        String::from("939"),
        String::from("7,13,x,x,59,x,31,19"),
    ];

     */
    solve_first(&lines);
    let lines = vec![
        vec![
            String::from("939"),
            String::from("7,13,x,x,59,x,31,19"),
        ],
        vec![
            String::from("939"),
            String::from("17,x,13,19"),
        ],
    //    vec![
    //        String::from("939"),
    //        String::from("67,7,59,61"),
    //    ],
        vec![
            String::from("939"),
            String::from("67,x,7,59,61"),
        ],
        vec![
            String::from("939"),
            String::from("67,7,x,59,61"),
        ],
        vec![
            String::from("939"),
            String::from("1789,37,47,1889"),
        ],
        vec![
            String::from("939"),
            String::from("17,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,739,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,971,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,19"),
        ],
    ];
    for line in &lines {
        solve_second(line);
    }
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
    println!("{:?}", &departures);
    println!("{:?}", colls);
    let mut it = 1;
    loop {
        let t = colls.0 * it - colls.1;
        if matches(t, &departures) {
            println!("12.2 = {:?}", t);
            break;
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

fn find_collisions(departures: &Vec<Departure>) -> (i64, i64) {
    let mut max_product = (departures[0].id + departures[departures.len() - 1].offset) * departures[departures.len() - 1].id;
    let mut max_offset = departures[departures.len() - 1].offset;
    (max_product, max_offset)
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