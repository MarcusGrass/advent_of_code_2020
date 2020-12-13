#![allow(dead_code)]
use dotenv;
mod util;
mod solutions;

/// Uses http to read challenge from https://adventofcode.com/2020/day/{day}/input
/// Needs the session (browser cookie) to fetch individual challenges
/// Put the session as SESSION={key} in a file named .env in the project root
/// Or just paste the String into String::from, I did it this way to avoid my session ending up
/// in the vcs.
fn main() {
    dotenv::dotenv().ok();
    let session = String::from(std::env::var("SESSION").unwrap());
    // solutions::aoc_two::solve_both(&session);
    // solutions::aoc_three::solve_both(&session);
    // solutions::aoc_four::solve_both(&session);
    // solutions::aoc_five::solve_both(&session);
    // solutions::aoc_six::solve_both(&session);
    // solutions::aoc_seven::solve_both(&session);
    // solutions::aoc_eight::solve_both(&session);
    // solutions::aoc_nine::solve_both(&session);
    // solutions::aoc_ten::solve_both(&session);
    // solutions::aoc_eleven::solve_both(&session);
    // solutions::aoc_twelve::solve_both(&session);
    // solutions::aoc_13::solve_both(&session);
    solutions::aoc_14::solve_both(&session);
}
