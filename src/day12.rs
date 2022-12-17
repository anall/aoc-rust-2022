#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc::{self, valid_neigbors_no_diagonal, CodeTimer};
use pathfinding::directed::astar::astar;
use rayon::prelude::*;
use std::io::BufRead;

fn route(
    start: &(usize, usize),
    end: &(usize, usize),
    map: &[Vec<u8>],
    m: usize,
    n: usize,
) -> Option<(Vec<(usize, usize)>, usize)> {
    astar(
        start,
        |node| {
            let cur = map[node.0][node.1];
            valid_neigbors_no_diagonal(*node, m, n)
                .into_iter()
                .filter_map(|neigh| {
                    let neigh_cell = map[neigh.0][neigh.1];
                    if neigh_cell <= cur || (neigh_cell - cur) == 1 {
                        Some((neigh, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<((usize, usize), usize)>>()
        },
        |node| node.0.abs_diff(end.0) + node.1.abs_diff(end.1),
        |node| node == end,
    )
}

fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let reader = aoc::file(filename)?;

    let mut timer = CodeTimer::new();

    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::with_capacity(line.len());
        for (j, mut ch) in line.bytes().enumerate() {
            if ch == b'S' {
                assert!(start.is_none());
                start = Some((i, j));
                ch = b'a';
            } else if ch == b'E' {
                assert!(end.is_none());
                end = Some((i, j));
                ch = b'z';
            }
            row.push(ch);
        }
        map.push(row);
    }

    timer.split("parse");

    let start = start.unwrap();
    let end = end.unwrap();
    let m = map.len();
    let n = map[0].len();

    let part1_route = route(&start, &end, &map, m, n).expect("route not found");

    timer.split("part1");

    // Brute force yaaaay
    let map_ref = &map; // we need to do this because Rust is being weird
    let part2 = (0..m)
        .into_par_iter()
        .flat_map(|i| {
            (0..m).into_par_iter().filter_map(move |j| {
                if map_ref[i][j] == b'a' {
                    route(&(i, j), &end, map_ref, m, n).map(|(_, len)| len)
                } else {
                    None
                }
            })
        })
        .min()
        .expect("shortest route not found");

    timer.stop("part2");

    Ok((part1_route.1, part2))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day12")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day12").unwrap();

        assert_eq!(part1, 31);
        assert_eq!(part2, 29);
    }
}
