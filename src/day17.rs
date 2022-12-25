#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unreadable_literal)]
use adventlib::aoc;
use std::{cmp, collections::HashMap, io::read_to_string};

const PIECES: [&[u8]; 5] = [
    &[0b00011110],
    &[0b00001000, 0b00011100, 0b00001000],
    &[0b00011100, 0b00000100, 0b00000100],
    &[0b00010000, 0b00010000, 0b00010000, 0b00010000],
    &[0b00011000, 0b00011000],
];

#[derive(Debug, Clone)]
struct Piece(Vec<u8>);
impl Piece {
    fn new(data: &[u8]) -> Self {
        Self(data.to_vec())
    }

    fn shift_using(&mut self, op: u8) -> bool {
        if op == b'<' {
            self.shift_left()
        } else if op == b'>' {
            self.shift_right()
        } else {
            unreachable!()
        }
    }

    fn unshift_using(&mut self, op: u8) -> bool {
        if op == b'>' {
            self.shift_left()
        } else if op == b'<' {
            self.shift_right()
        } else {
            unreachable!()
        }
    }

    fn shift_left(&mut self) -> bool {
        if self.0.iter().all(|v| v & 0b11000000 == 0) {
            for v in &mut self.0 {
                *v <<= 1;
            }
            true
        } else {
            false
        }
    }

    fn shift_right(&mut self) -> bool {
        if self.0.iter().all(|v| v & 0b1 == 0) {
            for v in &mut self.0 {
                *v >>= 1;
            }
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct PieceBag(u8);
impl PieceBag {
    fn new() -> Self {
        PieceBag(0)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn next(&mut self) -> Piece {
        let piece = Piece::new(PIECES[self.0 as usize]);
        self.0 = (self.0 + 1) % (PIECES.len() as u8);
        piece
    }
}

enum PlayfieldMoveState {
    Completed,
    Dropped,
}

#[derive(Debug)]
struct Playfield {
    field: Vec<u8>,
    first_empty: usize,
    bag: PieceBag,
    current_piece: Piece,
    current_height: usize,
    offset: usize,
}
impl Playfield {
    fn new() -> Self {
        let mut bag = PieceBag::new();
        let piece = bag.next();
        let mut rv = Self {
            field: vec![0b1111111],
            first_empty: 1,
            bag,
            current_piece: piece,
            current_height: 3 + 1,
            offset: 0,
        };
        rv.extend_if_needed();
        assert!(rv.piece_can_exist_at(rv.current_height));
        rv
    }

    fn piece_can_exist_at(&self, height: usize) -> bool {
        assert!(height - self.offset + self.current_piece.0.len() < self.field.len());
        (0..self.current_piece.0.len())
            .all(|i| self.field[height - self.offset + i] & self.current_piece.0[i] == 0)
    }

    fn extend_if_needed(&mut self) {
        let needed_length = self.first_empty + 3 + 4 + 1 - self.offset;
        let length = self.field.len();
        if let Some(to_add) = needed_length.checked_sub(length) {
            self.field.extend((0..to_add).map(|_| 0));
        }
    }

    fn drop_piece(&mut self) {
        let height = self.current_height;
        assert!(self.piece_can_exist_at(height));
        for i in 0..self.current_piece.0.len() {
            self.field[height - self.offset + i] |= self.current_piece.0[i];
        }
        self.first_empty = cmp::max(self.first_empty, height + self.current_piece.0.len());

        // check all pieces to see if there's a point that's closed off
        for i in height..self.first_empty - 1 {
            if i - self.offset >= 1
                && i >= self.offset
                && self.field[i - self.offset] | self.field[i - self.offset + 1] == 0b1111111
            {
                self.field.drain(0..(i - self.offset - 2));
                self.offset = i - 2;
                break;
            }
        }
        self.extend_if_needed();
        self.current_piece = self.bag.next();
        self.current_height = self.first_empty + 3;
    }

    fn process_move(&mut self, action: u8) -> PlayfieldMoveState {
        assert!(self.current_height > 0);
        assert!(self.piece_can_exist_at(self.current_height));
        if self.current_piece.shift_using(action) && !self.piece_can_exist_at(self.current_height) {
            self.current_piece.unshift_using(action);
        }
        if self.piece_can_exist_at(self.current_height - 1) {
            self.current_height -= 1;
            PlayfieldMoveState::Completed
        } else {
            self.drop_piece();

            PlayfieldMoveState::Dropped
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!("-------------");
        for (i, v) in self.field.iter().enumerate().rev() {
            let v = if i >= self.current_height
                && i < self.current_height + self.current_piece.0.len()
            {
                *v | self.current_piece.0[i - self.current_height]
            } else {
                *v
            };
            if i + self.offset == self.current_height {
                println!("{:8} {:08b} --- cur", i + self.offset, v);
            } else if i + self.offset == self.first_empty {
                println!("{:8} {:08b} --- empty", i + self.offset, v);
            } else {
                println!("{:8} {:08b}", i + self.offset, v);
            }
        }
    }
}

fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let moves = read_to_string(aoc::file(filename)?)?.into_bytes();
    let mut idx = 0;
    let mut timer = aoc::CodeTimer::new();

    let mut playfield = Playfield::new();
    let mut n_dropped: usize = 0;
    while n_dropped < 2022 {
        match playfield.process_move(moves[idx]) {
            PlayfieldMoveState::Completed => {}
            PlayfieldMoveState::Dropped => n_dropped += 1,
        }
        idx = (idx + 1) % moves.len();
    }

    timer.split("part1");
    let part1 = playfield.first_empty - 1;

    let mut seen: HashMap<(usize, Vec<u8>), (usize, usize)> = HashMap::new();
    let mut height_offset = None;
    let wanted_dropped = 1_000_000_000_000;
    while n_dropped < wanted_dropped {
        match playfield.process_move(moves[idx]) {
            PlayfieldMoveState::Completed => {}
            PlayfieldMoveState::Dropped => {
                n_dropped += 1;

                if height_offset.is_none() {
                    let state = (
                        idx,
                        playfield.field[0..playfield.first_empty - playfield.offset - 1].to_vec(),
                    );
                    if let Some(benchmark) = seen.get(&state) {
                        /*println!(
                            " at {} {}: current {} {}, seen {} {}",
                            idx,
                            playfield.bag.0,
                            playfield.first_empty - 1,
                            n_dropped,
                            benchmark.0,
                            benchmark.1
                        );*/
                        let drop_delta = n_dropped - benchmark.1;
                        let height_delta = playfield.first_empty - 1 - benchmark.0;
                        //println!(" in {} drops we grew {}", drop_delta, height_delta);
                        let drops_remain = wanted_dropped - n_dropped;
                        let whole_cycles = drops_remain / drop_delta;
                        /*println!(
                            " we need {} more drops, so {} cycles",
                            drops_remain, whole_cycles
                        );*/
                        n_dropped += drop_delta * whole_cycles;
                        height_offset = Some(height_delta * whole_cycles);
                    } else {
                        seen.insert(state, (playfield.first_empty - 1, n_dropped));
                    }
                }
            }
        }
        idx = (idx + 1) % moves.len();
    }
    let part2 = playfield.first_empty - 1 + height_offset.unwrap();
    timer.stop("part2");

    Ok((part1, part2))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day17")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day17").unwrap();

        assert_eq!(part1, 3068);
        assert_eq!(part2, 1514285714288);
    }
}
