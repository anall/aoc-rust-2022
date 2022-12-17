#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, io::BufRead, mem, usize};

lazy_static! {
    static ref MONKEY_HEADER: Regex = Regex::new(r"^Monkey (\d+):$").unwrap();
    static ref STARTING_ITEMS: Regex = Regex::new(r"^  Starting items: (.+)$").unwrap();
    static ref OPERATION_LINE: Regex = Regex::new(r"^  Operation: (.+)$").unwrap();
    static ref TEST_LINE: Regex = Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
    static ref TRUEFALSE_LINE: Regex =
        Regex::new(r"^    If (true|false): throw to monkey (\d+)$").unwrap();
    static ref OPERATION_ADD: Regex = Regex::new(r"^new = old \+ (\d+)$").unwrap();
    static ref OPERATION_MUL: Regex = Regex::new(r"^new = old \* (\d+)$").unwrap();
    static ref OPERATION_SQUARE: Regex = Regex::new(r"^new = old \* old$").unwrap();
}

type IntType = u64;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
    Add(IntType),
    Multiply(IntType),
    Square,
}
impl Operation {
    fn parse(op: &str) -> Option<Operation> {
        if let Some(cap) = OPERATION_ADD.captures(op) {
            Some(Operation::Add(cap[1].parse().unwrap()))
        } else if let Some(cap) = OPERATION_MUL.captures(op) {
            Some(Operation::Multiply(cap[1].parse().unwrap()))
        } else if OPERATION_SQUARE.is_match(op) {
            Some(Operation::Square)
        } else {
            None
        }
    }

    fn apply(self, old: IntType) -> IntType {
        match self {
            Operation::Add(v) => old.checked_add(v).unwrap(),
            Operation::Multiply(v) => old.checked_mul(v).unwrap(),
            Operation::Square => old.checked_mul(old).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: RefCell<Vec<IntType>>,
    operation: Operation,
    test_divisible: IntType,
    true_monkey: usize,
    false_monkey: usize,
}
impl Monkey {
    fn parse<I: Iterator<Item = String>>(iter: &mut I) -> Option<Self> {
        let Some(id) = MONKEY_HEADER.captures(&iter.next()?)
            .map(|cap| cap[1].parse::<usize>().unwrap()) else { return None };
        let Some(items) = STARTING_ITEMS.captures(&iter.next()?)
            .map(|cap| cap[1].split(", ").map(|part| part.parse().unwrap()).collect::<Vec<_>>() ) else { return None };
        let Some(operation) = OPERATION_LINE.captures(&iter.next()?)
            .map(|cap| Operation::parse(&cap[1]).unwrap() ) else { return None };
        let Some(test_divisible) = TEST_LINE.captures(&iter.next()?)
            .map(|cap| cap[1].parse().unwrap() ) else { return None };
        let Some(true_monkey) = TRUEFALSE_LINE.captures(&iter.next()?)
            .map(|cap| cap[2].parse().unwrap() ) else { return None };
        let Some(false_monkey) = TRUEFALSE_LINE.captures(&iter.next()?)
            .map(|cap| cap[2].parse().unwrap() ) else { return None };

        // maybe consume blank line;
        mem::drop(iter.next());

        Some(Monkey {
            id,
            items: RefCell::new(items),
            operation,
            test_divisible,
            true_monkey,
            false_monkey,
        })
    }

    fn take_items(&self) -> Vec<IntType> {
        let mut borrowed = self.items.borrow_mut();
        mem::take(borrowed.as_mut())
    }

    #[cfg(test)]
    fn test_items(&self) -> Vec<IntType> {
        self.items.borrow().clone()
    }

    fn thrown_item(&self, item: IntType) {
        self.items.borrow_mut().push(item);
    }
}

#[derive(Clone)]
struct GameState {
    monkies: Vec<Monkey>,
    items_considered: Vec<u64>,
    common_multiple: IntType,
}

impl GameState {
    fn parse<I: Iterator<Item = String>>(iter: &mut I) -> Self {
        let mut monkies = Vec::new();
        while let Some(monkey) = Monkey::parse(iter) {
            assert_eq!(monkey.id, monkies.len());
            monkies.push(monkey);
        }

        // FIXME: common_multiple should be lcm
        Self {
            items_considered: vec![0; monkies.len()],
            common_multiple: monkies.iter().map(|v| v.test_divisible).product(),
            monkies,
        }
    }

    fn from_file(filename: &str) -> aoc::Result<Self> {
        let reader = aoc::file(filename)?;
        let mut lines = reader.lines().map(Result::unwrap);

        Ok(GameState::parse(&mut lines))
    }

    fn run_round(&mut self, decrese_worry: bool) {
        for monkey in &self.monkies {
            for mut item in monkey.take_items() {
                self.items_considered[monkey.id] += 1;
                if decrese_worry {
                    item = monkey.operation.apply(item) / 3;
                } else {
                    item = monkey.operation.apply(item) % self.common_multiple;
                }
                if item % monkey.test_divisible == 0 {
                    self.monkies[monkey.true_monkey].thrown_item(item);
                } else {
                    self.monkies[monkey.false_monkey].thrown_item(item);
                }
            }
        }
    }
}
fn solve(filename: &str) -> aoc::Result<(u64, u64)> {
    let state = GameState::from_file(filename)?;

    let part1 = {
        let mut run_state = state.clone();
        for _ in 0..20 {
            run_state.run_round(true);
        }
        let mut items_considered = run_state.items_considered.clone();
        items_considered.sort_by_key(|k| std::cmp::Reverse(*k));
        items_considered[0] * items_considered[1]
    };

    let part2 = {
        let mut run_state = state;
        for _ in 0..10000 {
            run_state.run_round(false);
        }
        let mut items_considered = run_state.items_considered.clone();
        println!("{:?}", items_considered);
        items_considered.sort_by_key(|k| std::cmp::Reverse(*k));
        items_considered[0] * items_considered[1]
    };

    Ok((part1, part2))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day11")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{solve, GameState};

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day11").unwrap();

        assert_eq!(part1, 10605);
        assert_eq!(part2, 2713310158);
    }

    #[test]
    fn round_tests() {
        let mut state = GameState::from_file("inputs-sample/day11").unwrap();

        state.run_round(true);

        assert_eq!(state.monkies[0].test_items(), vec![20, 23, 27, 26]);
        assert_eq!(
            state.monkies[1].test_items(),
            vec![2080, 25, 167, 207, 401, 1046]
        );
        assert_eq!(state.monkies[2].test_items(), vec![]);
        assert_eq!(state.monkies[3].test_items(), vec![]);

        state.run_round(true);

        assert_eq!(state.monkies[0].test_items(), vec![695, 10, 71, 135, 350]);
        assert_eq!(state.monkies[1].test_items(), vec![43, 49, 58, 55, 362]);
        assert_eq!(state.monkies[2].test_items(), vec![]);
        assert_eq!(state.monkies[3].test_items(), vec![]);
    }
}
