use crate::solutions::aoc_eleven::SeatStatus::{OCCUPIED, EMPTY, NONE};

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(11, session);
    let seating = to_seat_status(&lines);
    solve_first(&seating);
    solve_second(&seating);
}

fn solve_first(seating: &Vec<Vec<SeatStatus>>) {
    let mut curr = seating.clone();
    loop {
        let res = transform(&curr, &num_adjacent_first, 4);
        if same(&curr, &res) {
            break;
        }
        curr = res;
    }
    println!("11.1 = {:?}", num_empty(&curr));
}

fn solve_second(seating: &Vec<Vec<SeatStatus>>) {
    let mut curr = seating.clone();
    loop {
        let res = transform(&curr, &num_adjacent_second, 5);
        if same(&curr, &res) {
            break;
        }
        curr = res;

    }
    println!("11.2 = {:?}", num_empty(&curr));
}

fn same(one: &Vec<Vec<SeatStatus>>, two: &Vec<Vec<SeatStatus>>) -> bool {
    for i in 0..one.len() {
        for j in 0..one[i].len() {
            if one[i][j] != two[i][j] {
                return false;
            }
        }
    }
    true
}

fn transform(seating: &Vec<Vec<SeatStatus>>, adj_fun: &dyn Fn((usize, usize), &Vec<Vec<SeatStatus>>) -> i32, cutoff: i32) -> Vec<Vec<SeatStatus>> {
    let mut transformed = Vec::new();
    for x in 0..seating.len() {
        transformed.push(Vec::new());
        for y in 0..seating[x].len() {
            if seating[x][y] == NONE {
                transformed.get_mut(x).unwrap().push(NONE);
                continue;
            }
            let adjacent: i32 = adj_fun((x, y), seating);
            if adjacent == 0 {
                transformed.get_mut(x).unwrap().push(OCCUPIED);
            } else if adjacent >= cutoff {
                transformed.get_mut(x).unwrap().push(EMPTY);
            } else {
                transformed.get_mut(x).unwrap().push(seating[x][y]);
            }
        }
    }
    transformed
}

fn num_empty(seating: &Vec<Vec<SeatStatus>>) -> i32 {
    seating.iter()
        .flat_map(|row| row.iter())
        .filter(|s| **s == OCCUPIED)
        .count() as i32
}

fn to_seat_status(lines: &Vec<String>) -> Vec<Vec<SeatStatus>> {
    let mut matrix = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for chr in line.chars() {
            row.push(
                match chr {
                    '#' => OCCUPIED,
                    'L' => EMPTY,
                    '.' => NONE,
                    _ => panic!("No match for char={}", chr)
                }
            )
        }
        matrix.push(row);
    }
    matrix
}

fn num_adjacent_first(coords: (usize, usize), seating: &Vec<Vec<SeatStatus>>) -> i32 {
    let mut empty = 0;
    for i in -1..2 as i32 {
        for j in -1..2 as i32 {
            let res_x = coords.0 as i32 + i;
            let res_y = coords.1 as i32 + j;
            if res_x < 0 || res_y < 0 {
                empty += 1;
                continue;
            }
            let x = res_x as usize;
            let y = res_y as usize;
            if x == seating.len() || y == seating[x].len() {
                empty += 1;
                continue;
            }
            if x == coords.0 && y == coords.1 {
                continue;
            }
            if seating[x][y] == EMPTY || seating[x][y] == NONE {
                empty += 1;
            }
        }
    }
    8 - empty
}


fn num_adjacent_second(coords: (usize, usize), seating: &Vec<Vec<SeatStatus>>) -> i32 {
    num_horizontal(coords, seating) + num_vertical(coords, seating) + num_diagonal(coords, seating)
}

fn num_horizontal(coords: (usize, usize), seating: &Vec<Vec<SeatStatus>>) -> i32 {
    let mut neighbours = 0;
    for y in coords.1 + 1..seating[coords.0].len() {
        if seating[coords.0][y] == OCCUPIED {

            neighbours += 1;
            break;
        } else if seating[coords.0][y] == EMPTY {
            break;
        }
    }
    for y in 0..coords.1 {
        if seating[coords.0][coords.1 - y - 1] == OCCUPIED {

            neighbours += 1;
            break;
        } else if seating[coords.0][coords.1 - y - 1] == EMPTY {
            break;
        }
    }
    neighbours
}

fn num_vertical(coords: (usize, usize), seating: &Vec<Vec<SeatStatus>>) -> i32 {
    let mut neighbours = 0;
    for x in coords.0 + 1..seating.len() {
        let seat = seating[x][coords.1];
        if seat == OCCUPIED {
            neighbours += 1;
            break;
        } else if seat == EMPTY {
            break;
        }
    }
    for x in 0..coords.0 {
        let seat = seating[coords.0 - x - 1][coords.1];
        if seat == OCCUPIED {
            neighbours += 1;
            break;
        } else if seat == EMPTY {
            break;
        }
    }
    neighbours
}

fn num_diagonal(coords: (usize, usize), seating: &Vec<Vec<SeatStatus>>) -> i32 {
    in_diag(&coords, (1, 1), seating) + in_diag(&coords, (1, -1), seating) +
        in_diag(&coords, (-1, -1), seating) + in_diag(&coords, (-1, 1), seating)
}

fn in_diag(coords: &(usize, usize), incr: (i32, i32), seating: &Vec<Vec<SeatStatus>>) -> i32 {
    let mut it = (coords.0 as i32, coords.1 as i32);
    let mut first_seen = &NONE;
    loop {
        it.0 += incr.0;
        it.1 += incr.1;
        let res = seating.get(it.0 as usize)
            .and_then(|o| o.get(it.1 as usize));
        if res.is_some() {
            let last = res.unwrap();
            if last != &NONE {
                first_seen = last;
                break;
            }
        } else {
            break;
        }
    }
    if first_seen == &OCCUPIED {
        return 1;
    }
    0
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum SeatStatus {
    EMPTY,
    OCCUPIED,
    NONE
}
