static ROWS: i32 = 128;
static COLS: i32 = 8;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(5, session);

    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let mut max = 0;
    let coords = get_all_seat_coords(lines);
    for coord in &coords {
        let result = coord.0 * 8 + coord.1;
        if result > max {
            max = result;
        }
    }

    println!("5.1 = {}", max);
}

fn solve_second(lines: &Vec<String>) {
    let coords = get_all_seat_coords(lines);
    let matrix = to_matrix(&coords);
    let mut last_possible_row = 0;
    for i in 1..matrix.len() {
        let mut zero_row = true;
        for val in matrix.get(i).unwrap() {
            if val != &0 {
                zero_row = false;
                break;
            }
        }
        if zero_row {
            last_possible_row = i - 2;
            break;
        }
    }
    for i in 1..last_possible_row + 1 {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                let result = i as i32 * COLS + j as i32;
                println!("5.2 = {}", result);
                return;
            }
        }
    }
}

fn get_all_seat_coords(lines: &Vec<String>) -> Vec<(i32, i32)> {
    let mut coords = Vec::new();
    for line in lines {
        let mut row_range = (0, 127);
        let mut col_range = (0, 7);
        for chr in line.chars() {
            let rows = row_range.1 - row_range.0 + 1;
            let cols = col_range.1 - col_range.0 + 1;
            if chr  == 'F' {
                row_range = (row_range.0, row_range.1 - rows / 2);
            } else if chr == 'B' {
                row_range = (row_range.0 + rows / 2, row_range.1);
            } else if chr == 'R' {
                col_range = (col_range.0 + cols / 2, col_range.1);
            } else if chr == 'L' {
                col_range = (col_range.0, col_range.1 - cols / 2);
            }
        }
        if row_range.0 == row_range.1 && col_range.0 == col_range.1 {
            coords.push((row_range.0, col_range.0));
        }
    }
    coords
}

fn to_matrix(coords: &Vec<(i32, i32)>) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; COLS as usize]; ROWS as usize];
    for coord in coords {
        match matrix.get_mut(coord.0 as usize) {
            Some(row) => row[coord.1 as usize] = 1,
            None => panic!("row coordinate pointed out of range {}", coord.0)
        }
    }
    matrix
}