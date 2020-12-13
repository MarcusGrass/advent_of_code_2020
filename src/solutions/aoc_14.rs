pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(14, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    println!("14.1 = {:?}", lines);
}

fn solve_second(lines: &Vec<String>) {
    println!("14.2 = {:?}", lines);
}