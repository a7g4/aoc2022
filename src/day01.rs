use anyhow::Result;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug)]
pub struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn from_lines<'a, Iter>(lines: Iter) -> Result<Elf>
    where
        Iter: Iterator<Item = &'a std::string::String>,
    {
        let calories: Result<Vec<i32>, _> = lines.map(|l| i32::from_str(&l)).collect();
        Ok(Elf {
            calories: calories?,
        })
    }
    
    fn total_calories(&self) -> i32 {
        self.calories.iter().sum()
    }
}

pub fn solve(lines : &Vec<std::string::String>) -> Result<()> {
    let elves: Result<Vec<Elf>> = lines
        .split(|line| line == "")
        .map(|lines| Elf::from_lines(lines.iter()))
        .collect();
    let elves = elves?;
    
    let mut total_calories : Vec<i32> = elves.iter().map(Elf::total_calories).collect();
    total_calories.sort();

    let max_calories = total_calories.iter().rev().next().unwrap();
    println!("{}", max_calories);

    let top_3_calories : i32 = total_calories.iter().rev().take(3).sum();
    println!("{}", top_3_calories);
    
    return Ok(());
}