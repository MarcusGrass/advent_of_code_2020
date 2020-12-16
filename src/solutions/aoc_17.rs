pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(17, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    println!("17.1 = {:?}", lines);
}

fn solve_second(lines: &Vec<String>) {
    println!("17.2 = {:?}", lines);
}
