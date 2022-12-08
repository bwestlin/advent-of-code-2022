use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<AssignmentPair>;

#[derive(Debug)]
struct AssignmentPair {
    a: Assignment,
    b: Assignment,
}

impl AssignmentPair {
    fn is_fully_containing(&self) -> bool {
        self.a.fully_contains(&self.b) || self.b.fully_contains(&self.a)
    }

    fn is_overlapping(&self) -> bool {
        self.a.overlaps(&self.b) || self.b.overlaps(&self.a)
    }
}

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn fully_contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.start <= other.end
            || self.end >= other.start && self.end <= other.end
    }
}

fn part1(input: &Input) -> usize {
    input.iter().filter(|a| a.is_fully_containing()).count()
}

fn part2(input: &Input) -> usize {
    input.iter().filter(|a| a.is_overlapping()).count()
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for AssignmentPair {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(AssignmentPair {
            a: parts.next().unwrap().parse::<Assignment>()?,
            b: parts.next().unwrap().parse::<Assignment>()?,
        })
    }
}

impl FromStr for Assignment {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        Ok(Assignment {
            start: parts.next().unwrap().parse::<u32>()?,
            end: parts.next().unwrap().parse::<u32>()?,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map(|line| Ok(line?.parse::<AssignmentPair>()?))
        .collect()
}

fn input() -> Result<Input> {
    let path = env::args()
        .nth(1)
        .with_context(|| "No input file given".to_owned())?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

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
        assert_eq!(part1(&as_input(INPUT)?), 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 4);
        Ok(())
    }
}
