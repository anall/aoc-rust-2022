#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Sand,
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Sand => 'o',
            }
        )
    }
}

fn solve(filename: &str) -> aoc::Result<(u32, u32)> {
    let reader = aoc::file(filename)?;

    let paths: Vec<Vec<(usize, usize)>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" -> ")
                .map(|point| {
                    let mut parts = point.split(",");
                    (
                        parts.next().unwrap().parse::<usize>().unwrap(),
                        parts.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for path in &paths {
        for (x, y) in path {
            min_x = cmp::min(min_x, *x);
            max_x = cmp::max(max_x, *x);
            max_y = cmp::max(max_y, *y);
        }
    }

    assert!(min_x > 50);
    min_x -= 50;
    max_x += 50;
    max_y += 1;
    assert!(min_x < max_x);

    let mut map: Vec<Vec<Tile>> = vec![vec![Tile::Empty; max_x - min_x]; max_y + 1];

    for path in paths {
        let mut iter = path.into_iter();
        let mut prev = iter.next().unwrap();
        for cur in iter {
            if prev.0 == cur.0 {
                for j in cmp::min(prev.1, cur.1)..=cmp::max(prev.1, cur.1) {
                    map[j][cur.0 - min_x] = Tile::Wall;
                }
            } else if prev.1 == cur.1 {
                for i in cmp::min(prev.0, cur.0)..=cmp::max(prev.0, cur.0) {
                    map[cur.1][i - min_x] = Tile::Wall;
                }
            } else {
                unimplemented!("diagonal line??")
            }
            prev = cur;
        }
    }

    for line in &map {
        print!("|");
        for ch in line {
            print!("{}",ch);
        }
        println!("|");
    }

    let mut n_landed = 0;
    let mut landed = true;
    let start_x = 500 - min_x;
    while landed {
        let mut cur_x = start_x;
        let mut cur_y = 0;
        landed = false;

        loop {
            assert!(cur_y < max_y);
            assert_eq!(map[cur_y][cur_x], Tile::Empty);
            if map[cur_y + 1][cur_x] == Tile::Empty {
                cur_y += 1;
            } else if map[cur_y + 1][cur_x - 1] == Tile::Empty {
                cur_x -= 1;
                cur_y += 1;
            } else if map[cur_y + 1][cur_x + 1] == Tile::Empty {
                cur_x += 1;
                cur_y += 1;
            } else {
                map[cur_y][cur_x] = Tile::Sand;
                n_landed += 1;
                landed = true;
                break;
            }
            if cur_y >= max_y {
                break;
            }
        }
    }

    let part1 = n_landed;


    Ok((part2, 0))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day14")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day14").unwrap();

        assert_eq!(part1, 24);
        assert_eq!(part2, 93);
    }
}
