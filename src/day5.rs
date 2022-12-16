#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use regex::Regex;
use std::{collections::VecDeque, io::BufRead};

fn solve(filename: &str) -> aoc::Result<(String, String)> {
    let reader = aoc::file(filename)?;
    let parse_regex = Regex::new(r"(?:\[(.)\]|   )(?: |$)").unwrap();
    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut lines = reader.lines();
    let mut has_capture = true;
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    while has_capture {
        has_capture = false;
        let line = lines.next().unwrap()?;
        for (i, cap) in parse_regex.captures_iter(&line).enumerate() {
            has_capture = true;
            if i >= stacks.len() {
                stacks.push(VecDeque::new());
            }
            if let Some(val) = cap.get(1) {
                stacks[i].push_front(val.as_str().chars().next().unwrap());
            }
        }
    }

    assert_eq!(lines.next().unwrap()?, "");

    let mut stacks2 = stacks.clone();

    for line in lines {
        let line = line.unwrap();
        let cap = move_regex
            .captures(&line)
            .expect("Failed to match move line");
        let n = cap[1].parse::<usize>().unwrap();
        let src = cap[2].parse::<usize>().unwrap() - 1;
        let dest = cap[3].parse::<usize>().unwrap() - 1;
        assert!(src < stacks.len());
        assert!(dest < stacks.len());

        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..n {
            let item = stacks[src].pop_back().unwrap();
            stacks[dest].push_back(item);

            tmp.push(stacks2[src].pop_back().unwrap());
        }
        for item in tmp.into_iter().rev() {
            stacks2[dest].push_back(item);
        }
    }

    Ok((
        stacks.iter().map(|v| v.back().unwrap()).collect(),
        stacks2.iter().map(|v| v.back().unwrap()).collect(),
    ))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day5")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day5").unwrap();

        assert_eq!(part1, "CMZ");
        assert_eq!(part2, "MCD");
    }
}
