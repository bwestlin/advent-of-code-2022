use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Option<u32>>;

fn solve(input: &Input) -> (u32, u32) {
    let mut cals = vec![];
    let mut curr = 0;
    for i in input {
        if let Some(i) = i {
            curr += i;
        } else {
            cals.push(curr);
            curr = 0;
        }
    }
    cals.push(curr);
    cals.sort();
    (*cals.last().unwrap(), cals.iter().rev().take(3).sum())
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map(|line| Ok(line?.parse::<u32>().ok()))
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

    const INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

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
        assert_eq!(solve(&as_input(INPUT)?).0, 24000);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(solve(&as_input(INPUT)?).1, 45000);
        Ok(())
    }
}
