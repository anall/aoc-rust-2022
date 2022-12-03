#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn parse(s : &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unimplemented!()
        }
    }

    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum GameResult {
    Player1,
    Player2,
    Tie
}
impl GameResult {
    fn score(self, perspective : GameResult) -> u32 {
        if self == perspective {
            6
        } else if self == GameResult::Tie {
            3
        } else {
            0
        }
    }

    fn parse(s : &str) -> Self {
        match s {
            "X" => Self::Player1,
            "Y" => Self::Tie,
            "Z" => Self::Player2,
            _ => unimplemented!()
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
struct Game(Move,Move,GameResult);


impl Game {
    fn parse(s : &str) -> Self {
        Self( Move::parse(&s[0..1]),Move::parse(&s[2..3]),GameResult::parse(&s[2..3]) )
    }

    fn result(self) -> GameResult {
        use Move::{Rock,Paper,Scissors};
        use GameResult::{Player1,Player2,Tie};

        #[allow(clippy::match_same_arms)]
        match self {    
            Self(a,b,_) if a == b => Tie,
            Self(Rock,Paper,_) => Player2,
            Self(Rock,Scissors,_) => Player1,
            Self(Paper,Rock,_) => Player1,
            Self(Paper,Scissors,_) => Player2,
            Self(Scissors,Rock,_) => Player2,
            Self(Scissors,Paper,_) => Player1,
            _ => unreachable!()
        }
    }

    #[allow(clippy::match_same_arms)]
     fn with_requred_result(self) -> Game {
        use Move::{Rock,Paper,Scissors};
        use GameResult::{Player1,Player2,Tie};

        Game(self.0,match self {
            Game(opp,_,Tie) => opp,
            Game(Rock,_,Player1) => Scissors,
            Game(Rock,_,Player2) => Paper,
            
            Game(Paper,_,Player1) => Rock,
            Game(Paper,_,Player2) => Scissors,
            
            Game(Scissors,_,Player1) => Paper,
            Game(Scissors,_,Player2) => Rock,
        },self.2)
     }
}



fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day2")?;

    let moves : Vec<_> =reader.lines().map(|line| Game::parse(&line.unwrap())).collect();

    println!("{:?}",moves.iter().map(|m| m.result().score(GameResult::Player2) + m.1.score()).sum::<u32>());
    println!("{:?}",moves.iter().map(|m| m.with_requred_result()).map(|m| m.result().score(GameResult::Player2) + m.1.score() ).sum::<u32>());

    Ok( () )
}