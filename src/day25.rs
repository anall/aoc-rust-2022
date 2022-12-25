#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use std::io::BufRead;

fn convert(number : &[u8]) -> i64 {
    let mut mul = 1;
    let mut result = 0;
    for val in number.into_iter().rev() {
        result += mul * match val {
            b'-' => -1,
            b'=' => -2,
            num => (num - b'0') as i64
        };
        mul *= 5;
    }

    result
}

fn unconvert(mut number : i64) -> Vec<u8> {
    let mut out : Vec<u8> = Vec::new();
    let mut borrowed = 0;

    while number > 0 {
        let cur_val = (number % 5) as u8 + borrowed;
        if cur_val > 2 {
            borrowed = 1;
            if cur_val == 3 {
                out.push(b'=')
            } else if cur_val == 4 {
                out.push(b'-')
            } else {
                unreachable!()
            }
        } else {
            borrowed = 0;
            out.push( cur_val + b'0' );
        }
        number /= 5;
    }
    out.reverse();
    
    out
}

fn solve(filename: &str) -> aoc::Result<(i64, String)> {
    let reader = aoc::file(filename)?;

    let sum = reader.lines().map(|v| convert(v.unwrap().as_bytes())).sum();
    let snafu_sum = unsafe { String::from_utf8_unchecked(unconvert(sum)) };

    assert_eq!( convert(snafu_sum.as_bytes()), sum );

    Ok( (sum,snafu_sum) )
}

fn main() -> aoc::Result<()> {
    let (part1_pre, part1) = solve("inputs/day25")?;

    println!("{}", part1_pre);
    println!("{}", part1);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{convert,solve};

    #[test]
    fn sample() {
        let (part1_pre, part1) = solve("inputs-sample/day25").unwrap();

        assert_eq!(part1_pre, 4890);
        assert_eq!(part1, "2=-1=0");
    }

    #[test]
    fn conversions() {
        assert_eq!( convert(b"1=-0-2"), 1747 );
        assert_eq!( convert(b"12111"), 906 );
        assert_eq!( convert(b"2=0="), 198 );
        assert_eq!( convert(b"21"), 11 );
        assert_eq!( convert(b"2=01"), 201 );
        assert_eq!( convert(b"111"), 31 );
        assert_eq!( convert(b"20012"), 1257 );
        assert_eq!( convert(b"112"), 32 );
        assert_eq!( convert(b"1=-1="), 353 );
        assert_eq!( convert(b"1-12"), 107 );
        assert_eq!( convert(b"12"), 7 );
        assert_eq!( convert(b"1="), 3 );
        assert_eq!( convert(b"122"), 37 );
    }
}
