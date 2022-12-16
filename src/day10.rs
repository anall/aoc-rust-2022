#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use console_bitmap::{draw_from_vec, BraillePatterns};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref NOOP_INSTRUCTION: Regex = Regex::new(r"^noop$").unwrap();
    static ref ADDX_INSTRUCTION: Regex = Regex::new(r"addx (-?\d+)$").unwrap();
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    #[allow(clippy::manual_map)]
    fn parse(line: &str) -> Option<Self> {
        if NOOP_INSTRUCTION.is_match(line) {
            Some(Instruction::Noop)
        } else if let Some(v) = ADDX_INSTRUCTION.captures(line) {
            Some(Instruction::AddX(v[1].parse::<i32>().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct RegisterFile {
    x: i32,
}

struct ExecutionContext<'a> {
    register: RegisterFile,
    program: &'a [Instruction],
    pc: usize,
    cycle: usize,
}

impl<'a> ExecutionContext<'a> {
    fn new(program: &'a [Instruction]) -> Self {
        Self {
            register: RegisterFile { x: 1 },
            program,
            pc: 0,
            cycle: 0,
        }
    }
}

impl<'a> Iterator for ExecutionContext<'a> {
    type Item = RegisterFile;

    fn next(&mut self) -> Option<Self::Item> {
        let register = self.register;
        if self.pc >= self.program.len() {
            Some(register)
        } else {
            match &self.program[self.pc] {
                Instruction::Noop => {
                    assert!(self.cycle == 0);
                    self.pc += 1;
                }
                Instruction::AddX(val) => {
                    if self.cycle == 1 {
                        self.register.x += val;
                        self.cycle = 0;
                        self.pc += 1;
                    } else {
                        self.cycle += 1;
                    }
                }
            }
            Some(register)
        }
    }
}

fn parse_program(filename: &str) -> aoc::Result<Vec<Instruction>> {
    let reader = aoc::file(filename)?;

    Ok(reader
        .lines()
        .map(|line| Instruction::parse(&line.unwrap()).unwrap())
        .collect())
}

fn solve(filename: &str) -> aoc::Result<(i32, Vec<Vec<bool>>)> {
    let program = parse_program(filename)?;

    Ok((part1(&program), part2(&program)))
}

#[allow(clippy::similar_names)]
fn part1(program: &[Instruction]) -> i32 {
    let mut execution = ExecutionContext::new(program);

    let at_20 = execution.nth(19).unwrap().x * 20;
    let at_60 = execution.nth(39).unwrap().x * 60;
    let at_100 = execution.nth(39).unwrap().x * 100;
    let at_140 = execution.nth(39).unwrap().x * 140;
    let at_180 = execution.nth(39).unwrap().x * 180;
    let at_220 = execution.nth(39).unwrap().x * 220;

    at_20 + at_60 + at_100 + at_140 + at_180 + at_220
}

fn part2(program: &[Instruction]) -> Vec<Vec<bool>> {
    let mut execution = ExecutionContext::new(program);
    let mut display = Vec::with_capacity(6);
    for _ in 0..6 {
        let mut line = Vec::with_capacity(40);
        for j in 0..40 {
            let position = execution.next().unwrap().x;
            line.push(j >= position - 1 && j <= position + 1);
        }
        display.push(line);
    }
    display
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day10")?;

    println!("{}", part1);

    for line in draw_from_vec::<BraillePatterns>(&part2) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{parse_program, solve, ExecutionContext};

    #[test]
    fn sample1() {
        let program = parse_program("inputs-sample/day10-sample1").unwrap();
        let mut execution = ExecutionContext::new(&program);

        assert_eq!(execution.next().unwrap().x, 1);
        // addx 3
        assert_eq!(execution.next().unwrap().x, 1);
        assert_eq!(execution.next().unwrap().x, 1);
        // addx -5
        assert_eq!(execution.next().unwrap().x, 4);
        assert_eq!(execution.next().unwrap().x, 4);

        assert_eq!(execution.next().unwrap().x, -1);
    }

    fn convert_display(line: &[bool]) -> String {
        let mut s = String::with_capacity(line.len());
        for v in line {
            s.push(if *v { '#' } else { '.' });
        }

        s
    }

    #[test]
    fn sample2() {
        let (part1, part2) = solve("inputs-sample/day10-sample2").unwrap();
        assert_eq!(part1, 13140);

        assert_eq!(
            convert_display(&part2[0]),
            "##..##..##..##..##..##..##..##..##..##.."
        );
        assert_eq!(
            convert_display(&part2[1]),
            "###...###...###...###...###...###...###."
        );
        assert_eq!(
            convert_display(&part2[2]),
            "####....####....####....####....####...."
        );
        assert_eq!(
            convert_display(&part2[3]),
            "#####.....#####.....#####.....#####....."
        );
        assert_eq!(
            convert_display(&part2[4]),
            "######......######......######......####"
        );
        assert_eq!(
            convert_display(&part2[5]),
            "#######.......#######.......#######....."
        );
    }
}
