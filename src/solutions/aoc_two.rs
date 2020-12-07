pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(2, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    println!("2.1 = {}", solve(lines, &is_valid_first));
}

fn solve_second(lines: &Vec<String>) {
    println!("2.2 = {}", solve(lines, &is_valid_second));
}

fn solve(lines: &Vec<String>, is_valid: &dyn Fn(i32, i32, &&str, &str) -> bool) -> i32 {
    let mut matched = 0;
    for s in lines {
        let part = s.split(" ");
        let col: Vec<&str> = part.collect();
        let range = to_range(col.get(0).unwrap());
        matched += range.zip(col.get(1).map(|l| {&l[..1]}))
            .zip(col.get(2))
            .and_then(|res| -> Option<i32> {
                if is_valid(res.0.0.0, res.0.0.1, res.1, res.0.1) {
                    return Some(1);
                }
                return Some(0);
            })
            .unwrap_or(0);
    }
    matched
}

fn is_valid_first(range_start: i32, range_end: i32, source: &&str, target: &str) -> bool {
    let num_matches = get_matches(source, target);
    num_matches >= range_start && num_matches <= range_end
}

fn is_valid_second(range_start: i32, range_end: i32, source: &&str, target: &str) -> bool {
    let mut chrs = source.chars();
    let at_first_index = chrs.nth(range_start as usize - 1).unwrap();
    let at_second_index = chrs.nth(range_end as usize - range_start as usize - 1).unwrap();
    let chr = target.chars().next().unwrap();
    let mut num_matches = 0;
    if chr == at_first_index {
        num_matches += 1;
    }
    if chr == at_second_index {
        num_matches += 1;
    }
    num_matches == 1
}


fn to_range(splt: &&str) -> Option<(i32, i32)> {
    let range_split: Vec<&str> = splt.split("-")
        .collect();
    let one = range_split.get(0)
        .and_then(|s| {s.parse().ok()});
    let two = range_split.get(1)
        .and_then(|s| {s.parse().ok()});
    one.zip(two)
}

fn get_matches(source: &&str, target: &str) -> i32 {
    let chrs = source.chars();
    let target = target.chars().next().unwrap();
    let mut count = 0;
    for chr in chrs {
        if chr == target {
            count += 1;
        }
    }
    count
}
