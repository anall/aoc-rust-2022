#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unimplemented!(),
        }
    }

    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GameResult {
    Player1,
    Player2,
    Tie,
}
impl GameResult {
    fn score(self, perspective: GameResult) -> u32 {
        if self == perspective {
            6
        } else if self == GameResult::Tie {
            3
        } else {
            0
        }
    }

    fn parse(s: &str) -> Self {
        match s {
            "X" => Self::Player1,
            "Y" => Self::Tie,
            "Z" => Self::Player2,
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Game(Move, Move, GameResult);

impl Game {
    fn parse(s: &str) -> Self {
        Self(
            Move::parse(&s[0..1]),
            Move::parse(&s[2..3]),
            GameResult::parse(&s[2..3]),
        )
    }

    fn result(self) -> GameResult {
        use GameResult::{Player1, Player2, Tie};
        use Move::{Paper, Rock, Scissors};

        #[allow(clippy::match_same_arms)]
        match self {
            Self(a, b, _) if a == b => Tie,
            Self(Rock, Paper, _) => Player2,
            Self(Rock, Scissors, _) => Player1,
            Self(Paper, Rock, _) => Player1,
            Self(Paper, Scissors, _) => Player2,
            Self(Scissors, Rock, _) => Player2,
            Self(Scissors, Paper, _) => Player1,
            _ => unreachable!(),
        }
    }

    #[allow(clippy::match_same_arms)]
    fn with_requred_result(self) -> Game {
        use GameResult::{Player1, Player2, Tie};
        use Move::{Paper, Rock, Scissors};

        Game(
            self.0,
            match self {
                Game(opp, _, Tie) => opp,
                Game(Rock, _, Player1) => Scissors,
                Game(Rock, _, Player2) => Paper,

                Game(Paper, _, Player1) => Rock,
                Game(Paper, _, Player2) => Scissors,

                Game(Scissors, _, Player1) => Paper,
                Game(Scissors, _, Player2) => Rock,
            },
            self.2,
        )
    }
}

fn solve(filename: &str) -> aoc::Result<(u32, u32)> {
    let reader = aoc::file(filename)?;

    let moves: Vec<_> = reader
        .lines()
        .map(|line| Game::parse(&line.unwrap()))
        .collect();

    Ok((
        moves
            .iter()
            .map(|m| m.result().score(GameResult::Player2) + m.1.score())
            .sum::<u32>(),
        moves
            .iter()
            .map(|m| m.with_requred_result())
            .map(|m| m.result().score(GameResult::Player2) + m.1.score())
            .sum::<u32>(),
    ))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day2")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day2").unwrap();

        assert_eq!(part1, 15);
        assert_eq!(part2, 12);
    }
}
