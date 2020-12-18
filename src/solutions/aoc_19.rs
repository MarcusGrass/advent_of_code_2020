pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(19, session);
    solve_first(&lines);
    solve_second(&lines);
}


fn solve_first(lines: &Vec<String>) {
    println!("18.1 = {:?}", lines);
}

fn solve_second(lines: &Vec<String>) {
    println!("18.2 = {:?}", lines);
}
