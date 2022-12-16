#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc::{self};
use serde::Deserialize;
use std::{cmp::Ordering, io::BufRead};

#[derive(Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum Value {
    Integer(u32),
    Divider(u32),
    List(Vec<Value>),
}
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(val) => write!(f, "{}", val),
            Self::Divider(val) => write!(f, "d:[[{}]]", val),
            Self::List(list) => write!(f, "{:?}", list),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::cmp::min;
        use Value::{Divider, Integer, List};

        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (List(l), List(r)) => {
                for i in 0..min(l.len(), r.len()) {
                    match l[i].cmp(&r[i]) {
                        Ordering::Equal => {}
                        order => return order,
                    }
                }
                l.len().cmp(&r.len())
            }
            (Integer(l), List(_)) => List(vec![Integer(*l)]).cmp(other),
            (List(_), Integer(r)) => self.cmp(&List(vec![Integer(*r)])),

            (Divider(v), _) => List(vec![List(vec![Integer(*v)])]).cmp(other),
            (_, Divider(v)) => self.cmp(&List(vec![List(vec![Integer(*v)])])),
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn correct_order(left: &Value, right: &Value) -> bool {
    matches!(left.cmp(right), Ordering::Less | Ordering::Equal)
}

fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let reader = aoc::file(filename)?;
    let mut lines = reader.lines();

    let mut packets = Vec::new();
    let mut i = 1;
    let mut part1 = 0;
    while let Some(Ok(left_str)) = lines.next() {
        let right_str = lines.next().unwrap()?;

        let left: Value = serde_json::from_str(&left_str).unwrap();
        let right: Value = serde_json::from_str(&right_str).unwrap();

        assert!(left.eq(&left));
        assert!(right.eq(&right));

        let _blank = lines.next();
        if correct_order(&left, &right) {
            part1 += i;
        }

        packets.push(left);
        packets.push(right);

        i += 1;
    }

    packets.push(Value::Divider(2));
    packets.push(Value::Divider(6));

    packets.sort();

    let part2 = packets
        .iter()
        .enumerate()
        .filter(|(_, el)| matches!(el, Value::Divider(_)))
        .map(|(idx, _)| idx + 1)
        .product();

    Ok((part1, part2))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day13")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::correct_order;
    use crate::{solve, Value};
    use std::cmp::Ordering;

    fn value(v: &str) -> Value {
        serde_json::from_str(&v).unwrap()
    }

    fn correct_order_helper(left_str: &str, right_str: &str) -> bool {
        let left: Value = serde_json::from_str(&left_str).unwrap();
        let right: Value = serde_json::from_str(&right_str).unwrap();

        correct_order(&left, &right)
    }

    fn order_helper(left_str: &str, right_str: &str) -> Ordering {
        let left: Value = serde_json::from_str(&left_str).unwrap();
        let right: Value = serde_json::from_str(&right_str).unwrap();

        left.cmp(&right)
    }

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day13").unwrap();

        assert_eq!(part1, 13);
        assert_eq!(part2, 140);
    }

    #[test]
    fn compare_one() {
        assert!(correct_order_helper("[1,1,3,1,1]", "[1,1,3,1,1]"));
        assert!(correct_order_helper("[[1],[2,3,4]]", "[[1],4]"));
        assert!(!correct_order_helper("[9]", "[[8,7,6]]"));
        assert!(correct_order_helper("[[4,4],4,4]", "[[4,4],4,4,4]"));
        assert!(!correct_order_helper("[7,7,7,7]", "[7,7,7]"));
        assert!(correct_order_helper("[]", "[3]"));
        assert!(!correct_order_helper("[[[]]]", "[[]]"));
    }

    #[test]
    fn deeper_tests() {
        assert_eq!(order_helper("[]", "[[]]"), Ordering::Less);
        assert_eq!(order_helper("[[]]", "[]"), Ordering::Greater);
        assert_eq!(order_helper("[[]]", "[[]]"), Ordering::Equal);

        assert_eq!(order_helper("[[]]", "[[[]]]"), Ordering::Less);
        assert_eq!(order_helper("[[[]]]", "[1,1,3,1,1]"), Ordering::Less);
    }

    #[test]
    fn simple_sort() {
        let mut v = Vec::new();
        let v1 = value("[]");
        let v2 = value("[[]]");
        let v3 = value("[[[]]]");
        let v4 = value("[1,1,3,1,1]");

        v.push(&v3);
        v.push(&v2);
        v.push(&v1);

        v.sort();

        assert_eq!(v[0], &v1);
        assert_eq!(v[1], &v2);
        assert_eq!(v[2], &v3);

        v.insert(1, &v4);

        v.sort();

        assert_eq!(v[0], &v1);
        assert_eq!(v[1], &v2);
        assert_eq!(v[2], &v3);
        assert_eq!(v[3], &v4);
    }
}
