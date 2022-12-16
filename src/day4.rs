#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use regex::Regex;
use std::io::BufRead;

struct Span(u32, u32);

impl Span {
    fn fully_contains(&self, other: &Self) -> bool {
        other.0 >= self.0 && other.1 <= self.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.0 >= self.0 && other.0 <= self.1 || other.1 >= self.0 && other.1 <= self.1
    }
}

fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let reader = aoc::file(filename)?;
    let parse_regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let spans: Vec<_> = reader
        .lines()
        .map(|d| {
            let line = d.unwrap();
            let captures = parse_regex.captures(&line).unwrap();
            let span1 = Span(captures[1].parse().unwrap(), captures[2].parse().unwrap());
            let span2 = Span(captures[3].parse().unwrap(), captures[4].parse().unwrap());

            (span1, span2)
        })
        .collect();

    Ok((
        spans
            .iter()
            .filter(|(span1, span2)| span1.fully_contains(span2) || span2.fully_contains(span1))
            .count(),
        spans
            .iter()
            .filter(|(span1, span2)| span1.overlaps(span2) || span2.overlaps(span1))
            .count(),
    ))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day4")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day4").unwrap();

        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }
}
