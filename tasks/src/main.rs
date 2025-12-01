mod day1;

use std::io::Result;

use crate::day1::{solve_1, solve_2};

fn main() -> Result<()> {
    // println!("{}", solve_1("./tasks/src/day1/input.txt")?);
    println!("{}", solve_2("./tasks/src/day1/input.txt")?);
    Ok(())
}
