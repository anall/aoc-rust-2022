#![warn(clippy::pedantic)]
use adventlib::aoc;
use std::{io::BufRead, mem};

#[derive(Debug)]
struct ElfData {
    total_calories: u32,
    //calories : Vec<u32>,
}

impl ElfData {
    fn new(calories: Vec<u32>) -> Self {
        Self {
            total_calories: calories.into_iter().sum(),
            //calories
        }
    }
}

fn solve(filename: &str) -> aoc::Result<(u32, u32)> {
    let reader = aoc::file(filename)?;

    let elves = {
        let mut elves: Vec<ElfData> = Vec::new();
        let mut current_elf: Vec<u32> = Vec::new();
        for line_result in reader.lines() {
            let line = line_result?;
            if line.is_empty() {
                elves.push(ElfData::new(mem::take(&mut current_elf)));
            } else {
                current_elf.push(line.parse::<u32>().unwrap());
            }
        }
        if !current_elf.is_empty() {
            elves.push(ElfData::new(current_elf));
        }

        elves.sort_by_key(|elf| std::cmp::Reverse(elf.total_calories));
        elves
    };

    Ok((
        elves[0].total_calories,
        elves[0..3]
            .iter()
            .map(|elf| elf.total_calories)
            .sum::<u32>(),
    ))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day1")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day1").unwrap();

        assert_eq!(part1, 24000);
        assert_eq!(part2, 45000);
    }
}
