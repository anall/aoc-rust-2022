#![warn( clippy::pedantic )]
use std::{io::BufRead, mem};
use adventlib::aoc;

#[derive(Debug)]
struct ElfData {
    total_calories : u32,
    //calories : Vec<u32>,
}

impl ElfData {
    fn new(calories : Vec<u32>) -> Self {
        Self {
            total_calories: calories.iter().sum(),
            //calories
        }
    }
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day1")?;

    let elves = {
        let mut elves : Vec<ElfData> = Vec::new();
        let mut current_elf : Vec<u32> = Vec::new();
        for line_result in reader.lines() {
            let line = line_result?;
            if line.len() == 0 {
                elves.push( ElfData::new( mem::replace(&mut current_elf,Vec::new()) ) );
            } else {
                current_elf.push( line.parse::<u32>().unwrap() );
            }
        }
        if ! current_elf.is_empty() {
            elves.push( ElfData::new(current_elf) );
        }

        elves.sort_by_key(|elf| std::cmp::Reverse(elf.total_calories));
        elves
    };

    //println!("{:?}",elves);

    println!("{}",elves[0].total_calories);
    println!("{}",elves[0..3].iter().map(|elf| elf.total_calories).sum::<u32>());

    Ok( () )
}