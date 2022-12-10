use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = String;

fn part1(input: &Input) -> usize {
    for i in 0..input.len() {
        let chrs = input.chars().skip(i).take(4).collect::<BTreeSet<_>>();
        if chrs.len() == 4 {
            return i + 4;
        }
    }
    0
}

fn part2(input: &Input) -> usize {
    for i in 0..input.len() {
        let chrs = input
            .chars()
            .cycle()
            .skip(i)
            .take(14)
            .collect::<BTreeSet<_>>();
        if chrs.len() == 14 {
            return i + 14;
        }
    }
    0
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
    reader.lines().map(|line| Ok(line?)).collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), 7);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 19);
        Ok(())
    }
}
