#![warn(clippy::pedantic)]
use adventlib::aoc::{
    self,
    point2d::{Direction, Point2D},
};
use std::convert::TryInto;
use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
struct Instruction(Direction, u32);

#[derive(Debug)]
struct Rope {
    head: Point2D,
    knots: Vec<Point2D>,
}
impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            head: Point2D(0, 0),
            knots: vec![Point2D::default(); knots - 1],
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn step(&mut self, direction: Direction) -> Point2D {
        // maybe verify the distance here?
        self.head += direction.delta(1);
        let mut prev = &self.head;

        for knot in &mut self.knots {
            let distance = prev - *knot;
            if distance.0.abs() == distance.1.abs() && distance.0.abs() <= 1 {
                // NOOP
            } else if distance.0.abs() + distance.1.abs() == 1 {
                // NOOP
            } else if distance.0 == 2 && distance.1 == 0 {
                knot.0 += 1;
            } else if distance.0 == -2 && distance.1 == 0 {
                knot.0 -= 1;
            } else if distance.0 == 0 && distance.1 == 2 {
                knot.1 += 1;
            } else if distance.0 == 0 && distance.1 == -2 {
                knot.1 -= 1;
            } else {
                knot.0 += distance.0.clamp(-1, 1);
                knot.1 += distance.1.clamp(-1, 1);
            }
            prev = knot;
        }
        *self.knots.last().unwrap()
    }
}

fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let reader = aoc::file(filename)?;
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let line = line?;
            let mut parts = line.split(' ');

            let direction = parts.next().unwrap().try_into()?;
            let count = parts.next().unwrap().parse()?;

            Ok(Instruction(direction, count))
        })
        .collect::<Result<Vec<Instruction>, aoc::Error>>()?;

    let mut short_visited: HashSet<Point2D> = HashSet::new();
    let mut long_visited: HashSet<Point2D> = HashSet::new();

    let mut short_rope = Rope::new(2);
    let mut long_rope = Rope::new(10);
    for inst in &instructions {
        for _ in 0..inst.1 {
            short_visited.insert(short_rope.step(inst.0));
            long_visited.insert(long_rope.step(inst.0));
        }
    }

    Ok((short_visited.len(), long_visited.len()))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day9")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample1() {
        let (part1, part2) = solve("inputs-sample/day9-sample1").unwrap();

        assert_eq!(part1, 13);
        assert_eq!(part2, 1);
    }

    #[test]
    fn sample2() {
        let (_, part2) = solve("inputs-sample/day9-sample2").unwrap();

        assert_eq!(part2, 36);
    }
}
