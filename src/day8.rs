#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use std::io::BufRead;

fn visible_trees_in<'a, I: IntoIterator<Item = &'a u8>>(cur_height: u8, iter: I) -> usize {
    let mut count = 0;
    for &height in iter {
        count += 1;
        if height >= cur_height {
            break;
        }
    }
    count
}

#[allow(clippy::many_single_char_names)]
fn visible_trees(data: &[Vec<u8>], i: usize, j: usize) -> usize {
    let cur_height = data[i][j];

    let l = visible_trees_in(cur_height, data[i][0..j].iter().rev());
    let r = visible_trees_in(cur_height, &data[i][j + 1..]);
    let u = visible_trees_in(cur_height, (0..i).rev().map(|v| &data[v][j]));
    let d = visible_trees_in(cur_height, (i + 1..data.len()).map(|v| &data[v][j]));

    u * d * l * r
}
fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let reader = aoc::file(filename)?;

    let grid: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().bytes().map(|byte| byte - b'0').collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // for each row walk across both ways, handling the visiblity grid
    for (i, row) in grid.iter().enumerate() {
        assert_eq!(row.len(), width);
        let mut left_tallest = 0;
        let mut right_tallest = 0;

        visible[i][0] = true;
        visible[i][width - 1] = true;

        for j in 0..width {
            let inv_j = width - j - 1;
            if row[j] > left_tallest {
                visible[i][j] = true;
                left_tallest = row[j];
            }
            if row[inv_j] > right_tallest {
                visible[i][inv_j] = true;
                right_tallest = row[inv_j];
            }
        }
    }

    // for each column walk across both ways, handling the visiblity grid
    for j in 0..width {
        let mut top_tallest = 0;
        let mut bottom_tallest = 0;

        visible[0][j] = true;
        visible[height - 1][j] = true;

        for i in 0..height {
            let inv_i = height - i - 1;
            if grid[i][j] > top_tallest {
                visible[i][j] = true;
                top_tallest = grid[i][j];
            }
            if grid[inv_i][j] > bottom_tallest {
                visible[inv_i][j] = true;
                bottom_tallest = grid[inv_i][j];
            }
        }
    }

    Ok((
        visible
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum(),
        (0..height)
            .map(|i| {
                (0..width)
                    .map(|j| visible_trees(&grid, i, j))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap(),
    ))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day8")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day8").unwrap();

        assert_eq!(part1, 21);
        assert_eq!(part2, 8);
    }
}
