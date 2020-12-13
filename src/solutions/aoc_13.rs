use crate::util::{modulo, modulo64};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::Vacant;
use modinverse::modinverse;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(13, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let schedule = to_schedule(&lines);
    let mut wait = i128::max_value();
    let mut id = 0i128;
    for bus in &schedule.buss_departures {
        let current = find_wait(&schedule.earliest_departure.clone(), bus);
        if wait > current {
            wait = current;
            id = bus.clone() as i128
        }
    }
    println!("13.1 = {:?}", wait * id);
}

fn find_wait(departure: &i128, bus_id: &i128) -> i128 {
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
    let colls = to_remainder_parts(&departures);
    let modulo = do_remainder(&colls);
    println!("13.2 = {:?}", modulo.0 - departures[departures.len() - 1].offset);
}

fn calculate(departures: &Vec<Departure>) -> i128 {
    let mut product = 1i128;
    for departure in departures {
        product *= (departure.id - departure.offset) as i128;
    }
    product
}

fn to_remainder_parts(departures: &Vec<Departure>) -> Vec<RemainderPart> {
    let max_offset = departures[departures.len() - 1].offset;
    departures.iter()
        .map(|d| RemainderPart{rem: modulo64(max_offset - d.offset, d.id), modulo: d.id})
        .collect()
}

fn do_remainder(remainders: &Vec<RemainderPart>) -> (i128, i128) {
    let mut parts: Vec<i128> = vec![0; remainders.len()];
    for i in 0..remainders.len() {
        let wanted_mod = remainders[i].modulo;
        for j in 0..remainders.len() {
            if i == j {
                continue;
            }
            if parts[j] == 0 {
                parts[j] += wanted_mod;
            } else {
                parts[j] *= wanted_mod;
            }
        }
    }
    for i in 0..remainders.len() {
        let wanted_mod = remainders[i].modulo;
        let wanted_rem = remainders[i].rem;
        let actual = modulo64(parts[i], wanted_mod);
        if actual != wanted_rem {
            parts[i] *= modinverse(actual, wanted_mod).unwrap() * wanted_rem;
        }

    }
    let mut sum = 0;
    for part in &parts {
        sum += part;
    }
    let mut last_mod = 1;
    for remainder in remainders {
        last_mod *= remainder.modulo;
    }
    (modulo64(sum, last_mod), last_mod)
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
    earliest_departure: i128,
    buss_departures: Vec<i128>
}

#[derive(Debug, Copy, Clone)]
struct Departure {
    id: i128,
    offset: i128
}

#[derive(Debug)]
struct RemainderPart {
    rem: i128,
    modulo: i128,
}