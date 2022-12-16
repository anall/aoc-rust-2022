#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
struct Item(u8);

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl Item {
    fn priority(self) -> u32 {
        if self.0 >= b'a' && self.0 <= b'z' {
            u32::from(self.0 - b'a' + 1)
        } else if self.0 >= b'A' && self.0 <= b'Z' {
            u32::from(self.0 - b'A' + 27)
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug)]
struct Rucksack(BTreeSet<Item>, BTreeSet<Item>);

impl Rucksack {
    fn parse(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mid = bytes.len() / 2;
        assert_eq!(mid * 2, bytes.len());

        Rucksack(
            bytes[0..mid].iter().copied().map(Item).collect(),
            bytes[mid..].iter().copied().map(Item).collect(),
        )
    }

    fn all_items(&self) -> impl Iterator<Item = Item> + '_ {
        self.0.union(&self.1).copied()
    }

    fn in_both(&self) -> Item {
        let mut intersect = self.0.intersection(&self.1);
        let result = intersect.next().unwrap();
        assert!(intersect.next().is_none());
        *result
    }
}
fn solve(filename: &str) -> aoc::Result<(u32, u32)> {
    let reader = aoc::file(filename)?;

    let sacks: Vec<_> = reader
        .lines()
        .map(|line| Rucksack::parse(&line.unwrap()))
        .collect();

    Ok((
        sacks
            .iter()
            .map(|sack| sack.in_both().priority())
            .sum::<u32>(),
        sacks
            .chunks(3)
            .map(|group| {
                let mut item_counts: BTreeMap<Item, u32> = BTreeMap::new();
                for sack in group {
                    for item in sack.all_items() {
                        *item_counts.entry(item).or_default() += 1;
                    }
                }
                item_counts.retain(|_, v| *v == 3);
                assert_eq!(item_counts.len(), 1);

                item_counts.keys().next().unwrap().priority()
            })
            .sum::<u32>(),
    ))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day3")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day3").unwrap();

        assert_eq!(part1, 157);
        assert_eq!(part2, 70);
    }
}
