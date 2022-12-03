use anyhow::Result;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub struct Game {
    opponent: Symbol,
    me: Symbol,
}

enum GameResult {
    Win,
    Loose,
    Draw,
}

impl Symbol {
    fn evaluate(&self, other: &Symbol) -> GameResult {
        match (self, other) {
            (Symbol::Rock, Symbol::Rock) => GameResult::Draw,
            (Symbol::Rock, Symbol::Paper) => GameResult::Loose,
            (Symbol::Rock, Symbol::Scissors) => GameResult::Win,
            (Symbol::Paper, Symbol::Rock) => GameResult::Win,
            (Symbol::Paper, Symbol::Paper) => GameResult::Draw,
            (Symbol::Paper, Symbol::Scissors) => GameResult::Loose,
            (Symbol::Scissors, Symbol::Rock) => GameResult::Loose,
            (Symbol::Scissors, Symbol::Paper) => GameResult::Win,
            (Symbol::Scissors, Symbol::Scissors) => GameResult::Draw,
        }
    }

    fn symbol_score(&self) -> i32 {
        match self {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        }
    }
    
    fn desired_state_from_opponent(&self, state : GameResult) -> Self {
        match (&self, state) {
            (_, GameResult::Draw) => self.clone(),
            (Symbol::Rock, GameResult::Win) => Symbol::Paper,
            (Symbol::Rock, GameResult::Loose) => Symbol::Scissors,            
            (Symbol::Paper, GameResult::Win) => Symbol::Scissors,            
            (Symbol::Paper, GameResult::Loose) => Symbol::Rock,            
            (Symbol::Scissors, GameResult::Win) => Symbol::Rock,            
            (Symbol::Scissors, GameResult::Loose) => Symbol::Paper,            
        }
    }
}

impl Game {
    fn score(&self) -> i32 {
        self.me.symbol_score()
            + match self.me.evaluate(&self.opponent) {
                GameResult::Win => 6,
                GameResult::Draw => 3,
                GameResult::Loose => 0,
            }
    }
}

impl Game {
    fn from_line(line: &String) -> Result<Self> {
        let mut splits = line.split(" ");
        let opponent: Option<Symbol> = match splits.next() {
            Some("A") => Some(Symbol::Rock),
            Some("B") => Some(Symbol::Paper),
            Some("C") => Some(Symbol::Scissors),
            Some(_) => None,
            None => None,
        };
        let opponent = opponent.unwrap();
        let me: Option<Symbol> = match splits.next() {
            Some("X") => Some(opponent.desired_state_from_opponent(GameResult::Loose)),
            Some("Y") => Some(opponent.desired_state_from_opponent(GameResult::Draw)),
            Some("Z") => Some(opponent.desired_state_from_opponent(GameResult::Win)),
            Some(_) => None,
            None => None,
        };
        Ok(Game {
            opponent: opponent,
            me: me.unwrap(),
        })
    }
}

pub fn solve(lines: &Vec<std::string::String>) -> Result<()> {
    // let games: Result<Vec<Game>> = lines.iter()
    //     .map(|line| Game::from_line(line))
    //     .map(Game::score)
    //     .collect();
    let score: i32 = lines
        .iter()
        .map(|line| Game::from_line(line))
        .map(|game| game.unwrap().score())
        .sum();
    println!("{:?}", score);
    Ok(())
}
