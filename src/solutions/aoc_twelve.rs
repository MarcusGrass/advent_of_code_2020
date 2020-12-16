use crate::solutions::aoc_twelve::Direction::{S, N, E, W, L, R, F};
use std::collections::HashMap;

const  DIRECTIONS: [Direction; 4] = [N, E, S, W];

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(12, session);
    let movements = to_movements(&lines);
    solve_first(&movements);
    solve_second(&movements);
}

fn solve_first(movements: &Vec<Movement>) {
    let mut accumulated = HashMap::new();
    accumulated.insert(N, 0);
    accumulated.insert(S, 0);
    accumulated.insert(W, 0);
    accumulated.insert(E, 0);
    let mut current_direction = E;
    for movement in movements {
        match movement.direction {
            F => accumulate(&Movement{direction: current_direction, amount: movement.amount}, &mut accumulated),
            N | S | W | E => accumulate(&movement, &mut accumulated),
            L | R => current_direction = turn(&current_direction, movement)
        }
    }
    let manhattan = (accumulated.get(&N).unwrap() - accumulated.get(&S).unwrap()).abs() +
        (accumulated.get(&E).unwrap() - accumulated.get(&W).unwrap()).abs();
    println!("12.1 = {:?}", manhattan);
}

fn accumulate(movement: &Movement, map: &mut HashMap<Direction, i32>) {
    let val = map.get(&movement.direction);
    let incr = val.unwrap() + movement.amount;
    map.insert(movement.direction.clone(), incr);
}

fn turn(current: &Direction, turn: &Movement) -> Direction {
    let mut clocks = turn.amount / 90;
    if turn.direction == L {
        clocks = -clocks;
    }
    let cur = DIRECTIONS.iter()
        .position(|r| r == current).unwrap();
    let ind = ((cur as i32 + clocks) % 4 + 4) % 4;
    DIRECTIONS[ind as usize]
}

fn solve_second(movements: &Vec<Movement>) {
    let mut waypoint = ((N, 1), (E, 10));
    let mut accumulated = HashMap::new();
    accumulated.insert(N, 0);
    accumulated.insert(E, 0);
    for movement in movements {
        match movement.direction {
            F => accumulate_second(movement.amount, &waypoint, &mut accumulated),
            N | E | S | W => waypoint = move_waypoint(movement, &waypoint),
            _ => waypoint = turn_waypoint(movement, &waypoint),
        }
    }
    let manhattan = accumulated.get(&N).unwrap().abs() + accumulated.get(&E).unwrap().abs();
    println!("12.2 = {:?}", manhattan);
}

fn accumulate_second(amount: i32, waypoint: &((Direction, i32), (Direction, i32)), map: &mut HashMap<Direction, i32>) {
    let val = map.get(&waypoint.0.0);
    let incr = val.unwrap() + waypoint.0.1 * amount;
    map.insert(waypoint.0.0.clone(), incr);
    let val = map.get(&waypoint.1.0);
    let incr = val.unwrap() + waypoint.1.1 * amount;
    map.insert(waypoint.1.0.clone(), incr);
}

fn move_waypoint(movement: &Movement, waypoint: &((Direction, i32), (Direction, i32))) -> ((Direction, i32), (Direction, i32)) {
    match movement.direction {
        N => ((waypoint.0.0, waypoint.0.1 + movement.amount), (waypoint.1.0, waypoint.1.1)),
        S => ((waypoint.0.0, waypoint.0.1 - movement.amount), (waypoint.1.0, waypoint.1.1)),
        E => ((waypoint.0.0, waypoint.0.1), (waypoint.1.0, waypoint.1.1 + movement.amount)),
        W => ((waypoint.0.0, waypoint.0.1), (waypoint.1.0, waypoint.1.1 - movement.amount)),
        _ => panic!()
    }
}

fn turn_waypoint(direction: &Movement, waypoint: &((Direction, i32), (Direction, i32))) -> ((Direction, i32), (Direction, i32)) {
    let dir1 = turn(&waypoint.0.0, direction);
    let mut mv1 = (Direction::N, 0);
    let dir2 = turn(&waypoint.1.0, direction);
    let mut mv2 = (Direction::N, 0);
    match dir1 {
        N => mv1 = (dir1, waypoint.0.1),
        S => mv1 = (N, -waypoint.0.1),
        E => mv2 = (dir1, waypoint.0.1),
        W => mv2 = (E, -waypoint.0.1),
        _ => panic!()
    }
    match dir2 {
        N => mv1 = (dir2, waypoint.1.1),
        S => mv1 = (N, -waypoint.1.1),
        E => mv2 = (dir2, waypoint.1.1),
        W => mv2 = (E, -waypoint.1.1),
        _ => panic!()
    }
    return (mv1, mv2)
}


fn to_movements(lines: &Vec<String>) -> Vec<Movement> {
    lines.iter()
        .map(|s| {
            let mut chars = s.chars();
            let direction = match chars.next().unwrap() {
                'N' => N,
                'S' => S,
                'E' => E,
                'W' => W,
                'L' => L,
                'R' => R,
                'F' => F,
                c => panic!("{} is no a valid direction", c)
            };
            let rest = chars.as_str();
            Movement{amount: rest.parse().unwrap(), direction}
        })
        .collect()
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Copy, Clone)]
struct Movement {
    amount: i32,
    direction: Direction,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
    L,
    R,
    F
}
