#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use std::io::BufRead;

fn solve(filename: &str) -> aoc::Result<(u32, u32)> {
    let reader = aoc::file(filename)?;

    Ok( (0,0) )
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day11")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day11").unwrap();

        assert_eq!(part1, 10605);
        assert_eq!(part2, 8);
    }
}
