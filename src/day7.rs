#![warn(clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]
use adventlib::aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;

lazy_static! {
    static ref CMD_CHDIR: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    static ref LS_DIR: Regex = Regex::new(r"^dir (.+)$").unwrap();
    static ref LS_FILE: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();
}

type Directory = Rc<RefCell<HashMap<String, DirectoryEntry>>>;

#[derive(Debug)]
enum DirectoryEntry {
    File(usize),
    Directory(Directory),
}

fn directory_size(dir_cell: &Directory) -> usize {
    let dir = dir_cell.borrow();
    dir.values()
        .map(|v| match v {
            DirectoryEntry::File(size) => *size,
            DirectoryEntry::Directory(subdir) => directory_size(subdir),
        })
        .sum()
}

fn solve(filename: &str) -> aoc::Result<(usize, usize)> {
    let reader = aoc::file(filename)?;

    let directory_root: Directory = Directory::default();
    let mut directory_tree: Vec<Directory> = vec![directory_root.clone()];
    let mut all_directories: Vec<Directory> = vec![directory_root.clone()];
    let mut cwd: Directory = directory_root.clone();

    let mut in_ls = false;

    for line in reader.lines() {
        let line = &line?;
        if let Some(x) = CMD_CHDIR.captures(line) {
            in_ls = false;
            if &x[1] == "/" {
                directory_tree = vec![directory_root.clone()];
            } else if x[1].contains('/') {
                unimplemented!("multi chdir");
            } else if &x[1] == ".." {
                directory_tree.pop();
            } else if let Some(DirectoryEntry::Directory(next_dir)) = cwd.borrow().get(&x[1]) {
                directory_tree.push(next_dir.clone());
            } else {
                panic!("missing direcrory {}", &x[1]);
            }
            cwd = directory_tree
                .last()
                .expect("somehow lost root directory")
                .clone();
        } else if line == "$ ls" {
            in_ls = true;
        } else if line.starts_with('$') {
            unimplemented!("invalid command {}", line);
        } else if !in_ls {
            unimplemented!("directory data outside ls");
        } else if let Some(x) = LS_DIR.captures(line) {
            let mut mut_cwd = cwd.borrow_mut();
            let new_dir: Directory = Directory::default();
            all_directories.push(new_dir.clone());
            mut_cwd.insert(x[1].to_string(), DirectoryEntry::Directory(new_dir));
        } else if let Some(x) = LS_FILE.captures(line) {
            let mut mut_cwd = cwd.borrow_mut();
            mut_cwd.insert(
                x[2].to_string(),
                DirectoryEntry::File(x[1].parse::<usize>().unwrap()),
            );
        } else {
            unimplemented!("line {}", line);
        }
    }

    let mut dir_sizes: Vec<usize> = all_directories.iter().map(directory_size).collect();

    let filesystem_size: usize = 70_000_000;
    let needed_space: usize = 30_000_000;
    let total_use = dir_sizes[0]; // we need this before we sort.

    dir_sizes.sort_unstable();

    let part1: usize = dir_sizes.iter().filter(|&&size| size <= 100_000).sum();

    let free_space = filesystem_size - total_use;
    let to_free = needed_space - free_space;

    let part2: usize = *dir_sizes.iter().find(|&&v| v > to_free).unwrap();

    Ok((part1, part2))
}

fn main() -> aoc::Result<()> {
    let (part1, part2) = solve("inputs/day7")?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn sample() {
        let (part1, part2) = solve("inputs-sample/day7").unwrap();

        assert_eq!(part1, 95437);
        assert_eq!(part2, 24933642);
    }
}
