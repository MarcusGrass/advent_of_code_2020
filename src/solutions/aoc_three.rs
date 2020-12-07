pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(3, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let matrix = create_matrix(lines);
    let slopes = vec![(3, 1)];
    println!("3.1 = {}", solve(matrix, slopes));
}

fn solve_second(lines: &Vec<String>) {
    let matrix = create_matrix(lines);
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!("3.2 = {}", solve(matrix, slopes));
}

fn solve(matrix: Vec<Vec<i32>>, slopes: Vec<(usize, usize)>) -> i64 {
    let mut coords = Vec::new();
    let len = matrix.get(0).unwrap().len();
    let mut results: Vec<i32> = Vec::new();
    for _i in 0..slopes.len() {
        coords.push((0, 0));
        results.push(0);
    }
    let mut done_indices: Vec<usize> = Vec::new();
    let mut done = 0;
    while done < slopes.len() {
        for i in 0..slopes.len() {
            if done_indices.contains(&i) {
                continue;
            }
            let mut current = coords.get_mut(i).unwrap();
            let slope = slopes.get(i).unwrap();
            current.0 += slope.0;
            if current.0 > len - 1 {
                current.0 -= len;
            }
            current.1 += slope.1;
            if current.1 > matrix.len() {
                done += 1;
                done_indices.push(i);
            }
            let current_result = results.get(i).unwrap();
            let new_result = current_result + matrix.get(current.1)
                .and_then(|row| {
                    row.get(current.0)
                })
                .unwrap_or(&0);
            results[i] = new_result;
        }
    }
    let mut total: i64 = 1;
    for result in results {
        total *= result as i64;
    }
    total

}

fn create_matrix(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(*&lines.len());
    for row in lines {
        let mut int_row = Vec::new();
        for chr in row.chars() {
            let s = chr.to_string();
            if s == "." {
                int_row.push(0);
            } else if s == "#" {
                int_row.push(1);
            } else {
                break;
            }
        }
        if int_row.len() > 0 {
            matrix.push(int_row);
        }
    }
    matrix
}

fn clone_owned(original: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    original.clone()
}
