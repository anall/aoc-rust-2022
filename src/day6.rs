#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use std::io::BufRead;

fn check_counts(counts: &[u8; 26]) -> bool {
    for &el in counts {
        if el > 1 {
            return false;
        }
    }

    true
}
fn find_marker(char_buf: &str, n_unique: usize) -> Option<usize> {
    let mut counts = [0u8; 26];
    let buf = char_buf.as_bytes();

    if buf.len() < n_unique {
        return None;
    }

    for i in 0..n_unique {
        counts[(buf[i] - b'a') as usize] += 1;
    }

    if check_counts(&counts) {
        return Some(n_unique);
    }

    for i in n_unique..buf.len() {
        assert!(counts[(buf[i - n_unique] - b'a') as usize] > 0);
        counts[(buf[i - n_unique] - b'a') as usize] -= 1;
        counts[(buf[i] - b'a') as usize] += 1;

        if check_counts(&counts) {
            return Some(i + 1);
        }
    }
    None
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day6")?;

    let data = reader.lines().next().unwrap().unwrap();

    println!("{}", find_marker(&data, 4).unwrap());
    println!("{}", find_marker(&data, 14).unwrap());
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::find_marker;

    #[test]
    fn part1() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));

        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));

        assert_eq!(find_marker("vwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(4));
    }

    #[test]
    fn part2() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }
}
