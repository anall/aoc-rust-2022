#![warn( clippy::pedantic )]
use std::io::BufRead;
use std::collections::{BTreeSet,BTreeMap};
use adventlib::aoc;
use itertools::Itertools;

#[derive(Hash,PartialEq, PartialOrd, Eq, Ord,Copy,Clone)]
struct Item(u8);

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0 as char)
    }
}

impl Item {
    fn priority(&self) -> u32 {
        if self.0 >= b'a' && self.0 <= b'z' {
            (self.0 - b'a' + 1) as u32
        } else if self.0 >= b'A' && self.0 <= b'Z' {
            (self.0 - b'A' + 27) as u32
        } else {
            unreachable!()
        }
    }
}
#[derive(Debug)]
struct Rucksack(BTreeSet<Item>,BTreeSet<Item>);

impl Rucksack {
    fn parse(s : &str) -> Self {
        let bytes = s.as_bytes();
        let mid = bytes.len() / 2;
        assert_eq!(mid*2, bytes.len());

        Rucksack( bytes[0..mid].iter().copied().map(Item).collect(), bytes[mid..].iter().copied().map(Item).collect() )
    }

    fn all_items(&self) -> impl Iterator<Item=Item> + '_ {
        self.0.union(&self.1).copied()
    }
    
    fn in_both(&self) -> Item {
        let mut intersect = self.0.intersection(&self.1);
        let result = intersect.next().unwrap();
        assert!(intersect.next().is_none());
        *result
    }
}
fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day3")?;

    let sacks : Vec<_> = reader.lines().map(|line| Rucksack::parse( &line.unwrap() ) ).collect();

    println!("{}",sacks.iter().map(|sack| sack.in_both().priority()).sum::<u32>());

    println!("{}",sacks.chunks(3).map(|group| {
        let mut item_counts : BTreeMap<Item,u32> = BTreeMap::new();
        for sack in group {
            for item in sack.all_items() {
                *item_counts.entry(item).or_default() += 1;
            }
        }
        item_counts.retain(|_,v| *v == 3);
        assert_eq!(item_counts.len(),1);

        item_counts.keys().next().unwrap().priority()
    }).sum::<u32>());

    Ok( () )
}