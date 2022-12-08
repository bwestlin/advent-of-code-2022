use std::cell::RefCell;
use std::env;
use std::fmt::Debug;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;

use anyhow::{Context, Result};

use utils::measure;

type Input = Rc<RefCell<Box<Directory>>>;

struct Directory {
    parent: Option<Rc<RefCell<Box<Directory>>>>,
    name: String,
    dirs: Vec<Rc<RefCell<Box<Directory>>>>,
    files: Vec<File>,
    cached_size: RefCell<Option<u32>>,
}

impl Directory {
    fn new(parent: Rc<RefCell<Box<Directory>>>, name: &str) -> Self {
        Self {
            parent: Some(parent),
            name: name.to_owned(),
            dirs: vec![],
            files: vec![],
            cached_size: RefCell::new(None),
        }
    }

    fn root() -> Self {
        Self {
            parent: None,
            name: "/".to_owned(),
            dirs: vec![],
            files: vec![],
            cached_size: RefCell::new(None),
        }
    }

    fn size(&self) -> u32 {
        let maybe_size = self.cached_size.borrow();
        if let Some(size) = maybe_size.as_ref() {
            *size
        } else {
            drop(maybe_size);
            let mut size = 0;
            for dir in &self.dirs {
                size += dir.borrow().size();
            }
            for file in &self.files {
                size += file.size;
            }
            *self.cached_size.borrow_mut() = Some(size);
            size
        }
    }

    fn visit<F>(&self, visitor: &mut F)
    where
        F: FnMut(&Self),
    {
        visitor(self);
        for dir in &self.dirs {
            dir.borrow().visit(visitor);
        }
    }
}

impl Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Avoid printing parent as it will cause endless loop
        f.debug_struct("Directory")
            .field("name", &self.name)
            .field("dirs", &self.dirs)
            .field("files", &self.files)
            .field("cached_size", &self.cached_size)
            .finish()
    }
}

#[derive(Debug)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: u32,
}

fn part1(input: &Input) -> u32 {
    let mut sum = 0;
    input.borrow().visit(&mut |dir: &Directory| {
        let size = dir.size();
        if size < 100000 {
            sum += size;
        }
    });
    sum
}

fn part2(input: &Input) -> u32 {
    let unused_space = 70000000 - input.borrow().size();
    let needed_space = 30000000 - unused_space;

    let mut least_needed = input.borrow().size();

    input.borrow().visit(&mut |dir: &Directory| {
        let size = dir.size();
        if size >= needed_space && size < least_needed {
            least_needed = size;
        }
    });
    least_needed
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let root_dir = Rc::new(RefCell::new(Box::new(Directory::root())));
    let mut curr_dir = root_dir.clone();

    for line in reader.lines() {
        let line = line?;

        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        match parts[..] {
            ["$", "cd", "/"] => curr_dir = root_dir.clone(),
            ["$", "cd", ".."] => {
                let maybe_dir = curr_dir.try_borrow()?.parent.clone();
                if let Some(dir) = maybe_dir {
                    curr_dir = dir;
                }
            }
            ["$", "cd", name] => {
                let maybe_idx = curr_dir
                    .try_borrow()?
                    .dirs
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.borrow().name == name)
                    .map(|(i, _)| i);

                let idx = if let Some(idx) = maybe_idx {
                    idx
                } else {
                    let parent = curr_dir.clone();
                    let mut curr_dir = curr_dir.try_borrow_mut()?;

                    let dir = Rc::new(RefCell::new(Box::new(Directory::new(parent, name))));

                    curr_dir.dirs.push(dir.clone());
                    curr_dir.dirs.len() - 1
                };

                let dir = curr_dir.try_borrow()?.dirs[idx].clone();
                curr_dir = dir;
            }
            ["$", "ls"] => {}
            ["dir", name] => {
                let parent = curr_dir.clone();
                let mut curr_dir = curr_dir.try_borrow_mut()?;

                let dir = Rc::new(RefCell::new(Box::new(Directory::new(parent, name))));

                curr_dir.dirs.push(dir.clone());
            }
            [size, name] => {
                let mut curr_dir = curr_dir.try_borrow_mut()?;
                curr_dir.files.push(File {
                    name: name.to_owned(),
                    size: size.parse::<u32>()?,
                });
            }
            _ => anyhow::bail!("Unhandled {:?}", parts),
        }
    }

    Ok(root_dir)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(std::fs::File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                .skip(1)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), 95437);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 24933642);
        Ok(())
    }
}
