#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc::{self, point2d::Point2D, CodeTimer};
use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{cmp, io::BufRead};

lazy_static! {
    static ref PARSE_REGEX: Regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
}

#[derive(Debug)]
struct SensorExtents {
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
    center_y: i64,
}
impl SensorExtents {
    #[inline]
    fn is_y_within(&self, y: i64) -> bool {
        y >= self.min_y && y <= self.max_y
    }

    #[inline]
    fn x_extents_at(&self, y: i64) -> Option<(i64, i64)> {
        if self.is_y_within(y) {
            let y_dist = (self.center_y - y).abs();
            Some((self.min_x + y_dist, self.max_x - y_dist))
        } else {
            None
        }
    }
}
#[derive(Debug)]
struct Sensor {
    //_sensor: Point2D,
    //_beacon: Point2D,
    extents: SensorExtents,
}
impl Sensor {
    fn parse(line: Result<String, std::io::Error>) -> aoc::Result<Self> {
        let line = line?;
        if let Some(cap) = PARSE_REGEX.captures(&line) {
            let sensor = Point2D(cap[1].parse()?, cap[2].parse()?);
            let beacon = Point2D(cap[3].parse()?, cap[4].parse()?);

            let distance = (sensor - beacon).manhattan_distance();
            let extents = SensorExtents {
                min_x: sensor.0 - distance,
                max_x: sensor.0 + distance,
                min_y: sensor.1 - distance,
                max_y: sensor.1 + distance,
                center_y: sensor.1,
            };
            Ok(Sensor {
                /*_sensor: sensor, _beacon: beacon,*/ extents,
            })
        } else {
            Err(aoc::Error::ParseFailed)
        }
    }

    fn extents(&self) -> &SensorExtents {
        &self.extents
    }
}

#[derive(Debug, Clone)]
struct IntSpan {
    ranges: Vec<(i64, i64)>,
}
impl IntSpan {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn add_range(&mut self, min: i64, max: i64) {
        if self.ranges.is_empty() {
            self.ranges.push((min, max));
        } else {
            // find insertion point
            let insert_pos = match self.ranges.binary_search_by(|v| v.0.cmp(&min)) {
                Ok(v) | Err(v) => v,
            };
            self.ranges.insert(insert_pos, (min, max));
            // now fixup
            let mut cur_i = insert_pos;
            while cur_i < self.ranges.len() {
                // did we just encroach on the next element?
                if cur_i + 1 < self.ranges.len()
                    && self.ranges[cur_i + 1].0 <= self.ranges[cur_i].1 + 1
                {
                    // extend ourselves and delete the next element
                    self.ranges[cur_i].1 = cmp::max(self.ranges[cur_i].1, self.ranges[cur_i + 1].1);
                    self.ranges.remove(cur_i + 1);
                } else if cur_i > 0 && self.ranges[cur_i - 1].1 + 1 >= self.ranges[cur_i].0 {
                    // extend previous element and delete ourselves
                    self.ranges[cur_i - 1].1 =
                        cmp::max(self.ranges[cur_i - 1].1, self.ranges[cur_i].1);
                    self.ranges.remove(cur_i);
                } else {
                    cur_i += 1;
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.ranges.len()
    }
}

fn solve(filename: &str, part1_y: i64, part2_extent: i64) -> aoc::Result<(i64, i64)> {
    let mut timer = CodeTimer::new();

    let reader = aoc::file(filename)?;
    let sensors: Vec<Sensor> = reader
        .lines()
        .map(Sensor::parse)
        .collect::<aoc::Result<_>>()?;

    timer.split("parse");

    let part1 = {
        let mut spans = IntSpan::new();
        for sensor in &sensors {
            let extents = sensor.extents();
            if extents.is_y_within(part1_y) {
                let x_extents_at = extents.x_extents_at(part1_y).unwrap();
                //println!("adding extent {:?}",x_extents_at);
                spans.add_range(x_extents_at.0, x_extents_at.1);
            }
        }
        spans.ranges.into_iter().map(|span| span.1 - span.0).sum()
    };

    timer.split("part1");

    // debugging blah
    /*{
        for y in 0..=part2_extent {
            let mut row : Vec<char> = vec!['.'; (part2_extent as usize+1)];
            for sensor in &sensors {
                let extents = sensor.extents();
                if extents.is_y_within(y) {
                    let x_extents_at = extents.x_extents_at(y).unwrap();
                    for x in 0..=part2_extent {
                        if x >= x_extents_at.0 && x <= x_extents_at.1 {
                            row[x as usize] = '#';
                        }
                    }
                }
            }
            println!("{}",row.into_iter().collect::<String>());
        }
        timer.split("debug");
    }*/

    let part2 = {
        (0..=part2_extent).into_par_iter().find_map_any(|y| {
            let mut spans = IntSpan::new();
            for sensor in &sensors {
                let extents = sensor.extents();
                if extents.is_y_within(y) {
                    let x_extents_at = extents.x_extents_at(y).unwrap();
                    //println!("adding extent {:?}",x_extents_at);
                    spans.add_range(x_extents_at.0, x_extents_at.1);
                }
            }
            if spans.len() > 1 {
                assert!(spans.len() == 2);

                Some( (spans.ranges[0].1 + 1) * 4_000_000 + y)
            } else {
                None
            }
        }).unwrap()
    };

    timer.stop("part2");

    Ok((part1, part2))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day15", 2_000_000, 4_000_000)?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{solve, IntSpan};

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day15", 10, 20).unwrap();

        assert_eq!(part1, 26);
        assert_eq!(part2, 56000011);
    }

    #[test]
    fn intspan() {
        let mut span = IntSpan::new();

        assert_eq!(span.len(), 0);

        // 5-10
        span.add_range(5, 10);
        assert_eq!(span.ranges, vec![(5, 10)]);

        // 5-10, 20-30
        span.add_range(20, 30);
        assert_eq!(span.ranges, vec![(5, 10), (20, 30)]);

        // 1-10, 20-30
        span.add_range(1, 5);
        assert_eq!(span.ranges, vec![(1, 10), (20, 30)]);

        // 1-30 ( now let's try subsuming the entire range )
        span.add_range(1, 21);
        assert_eq!(span.ranges, vec![(1, 30)]);

        // 1-50
        span.add_range(29, 50);
        assert_eq!(span.ranges, vec![(1, 50)]);

        // 1-50,75-100
        span.add_range(75, 100);
        assert_eq!(span.ranges, vec![(1, 50), (75, 100)]);

        // 1-50,60-70,75-100
        span.add_range(60, 70);
        assert_eq!(span.ranges, vec![(1, 50), (60, 70), (75, 100)]);

        // 1-100
        span.add_range(50, 75);
        assert_eq!(span.ranges, vec![(1, 100)]);

        // 1-200
        span.add_range(101, 200);
        assert_eq!(span.ranges, vec![(1, 200)]);

        // (-200)-200
        span.add_range(-200, 0);
        assert_eq!(span.ranges, vec![(-200, 200)]);
    }

    #[test]
    fn intspan_real() {
        let mut span = IntSpan::new();

        assert_eq!(span.len(), 0);

        // (-12)-2
        span.add_range(-12, 2);
        assert_eq!(span.ranges, vec![(-12, 2)]);

        // (-12)-2,5-13
        span.add_range(5, 13);
        assert_eq!(span.ranges, vec![(-12, 2), (5, 13)]);

        // (-12)-13
        span.add_range(-5, 13);
        assert_eq!(span.ranges, vec![(-12, 13)]);

        // (-12)-13
        span.add_range(-3, 3);
        assert_eq!(span.ranges, vec![(-12, 13)]);

        // (-12)-25
        span.add_range(9, 25);
        assert_eq!(span.ranges, vec![(-12, 25)]);

        // (-12)-25
        span.add_range(7, 17);
        assert_eq!(span.ranges, vec![(-12, 25)]);
    }
}
