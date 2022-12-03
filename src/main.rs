use anyhow::Result;
use std::io;
use std::vec::Vec;

mod day02;

fn main() -> Result<()> {
    let lines: Result<Vec<std::string::String>, _> = io::stdin().lines().collect();
    let lines = lines?;
    day02::solve(&lines)?;
    Ok(())
}
