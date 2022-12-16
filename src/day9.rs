#![warn(clippy::pedantic)]
use adventlib::aoc::{self,point2d::{Point2D,Direction}};
use std::{io::BufRead, collections::HashSet};
use std::convert::TryInto;

#[derive(Debug)]
struct Instruction(Direction,u32);

#[derive(Default,Debug)]
struct Rope {
    head : Point2D,
    tail : Point2D
}
impl Rope {
    fn step(&mut self, direction : Direction) -> Point2D {
        // maybe verify the distance here?
        self.head += direction.delta(1);

        let distance = self.head - self.tail;
        if distance.0.abs() == distance.1.abs() && distance.0.abs() == 1 {
            // NOOP
        } else if distance.0.abs() + distance.1.abs() <= 1 {
            // NOOP
        } else if distance.0 == 2 && distance.1 == 0 {
            self.tail.0 += 1;
        } else if distance.0 == -2 && distance.1 == 0 {
            self.tail.0 -= 1;
        } else if distance.0 == 0 && distance.1 == 2 {
            self.tail.1 += 1;
        } else if distance.0 == 0 && distance.1 == -2 {
            self.tail.1 -= 1;
        } else if distance.0.abs() + distance.1.abs() == 3 {
            self.tail.0 += distance.0.clamp(-1,1);
            self.tail.1 += distance.1.clamp(-1,1);
        } else {
            unimplemented!();
        }

        self.tail
    }
}


fn solve(filename : &str) -> aoc::Result<(usize,u32)> {
    let reader = aoc::file(filename)?;
    let instructions : Vec<Instruction> = reader.lines().map(|line| {
        let line = line?;
        let mut parts = line.split(' ');

        let direction = parts.next().unwrap().try_into()?;
        let count = parts.next().unwrap().parse()?;

        Ok( Instruction( direction, count ) )
    }).collect::<Result<Vec<Instruction>,aoc::Error>>()?;

    let mut visited : HashSet<Point2D> = HashSet::new();
    let mut rope = Rope::default();
    for inst in &instructions {
        for _ in 0..inst.1 {
            let new_tail = rope.step(inst.0);
            visited.insert(new_tail);
        }
    }

    Ok( (visited.len(),0) )
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
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day9").unwrap();

        assert_eq!(part1, 13);
        assert_eq!(part2, 8);
    }
}

