use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

#[derive(Debug)]
struct Input {
    stacks: Vec<String>,
    procedure: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    num: usize,
    from_idx: usize,
    to_idx: usize,
}

fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for Step {
        num,
        from_idx,
        to_idx,
    } in &input.procedure
    {
        for _ in 0..*num {
            let c = stacks[*from_idx].pop().unwrap();
            stacks[*to_idx].push(c);
        }
    }

    top_letters(stacks)
}

fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    let mut buf = String::new();

    for Step {
        num,
        from_idx,
        to_idx,
    } in &input.procedure
    {
        buf.clear();
        for _ in 0..*num {
            let c = stacks[*from_idx].pop().unwrap();
            buf.push(c);
        }
        for c in buf.chars().rev() {
            stacks[*to_idx].push(c);
        }
    }

    top_letters(stacks)
}

fn top_letters(stacks: Vec<String>) -> String {
    stacks
        .iter()
        .filter_map(|s| s.chars().rev().next())
        .collect()
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for Step {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_ascii_whitespace();
        let splits = splits.by_ref();
        let num = splits.nth(1).context("No num")?.parse::<usize>()?;
        let from_idx = splits.nth(1).context("No from")?.parse::<usize>()? - 1;
        let to_idx = splits.nth(1).context("No to")?.parse::<usize>()? - 1;
        Ok(Step {
            num,
            from_idx,
            to_idx,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines();

    fn parse_stack_pos(s: &str) -> Option<char> {
        if s.starts_with('[') {
            s.chars().nth(1)
        } else {
            None
        }
    }

    let mut stacks = vec![];

    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut curr = line.as_str();
        let mut row = vec![];
        while !curr.is_empty() {
            let eval = &curr[0..3];
            row.push(parse_stack_pos(eval));
            if curr.len() <= 4 {
                break;
            }
            let next = &curr[4..];
            curr = next;
        }

        if stacks.is_empty() {
            for _ in 0..row.len() {
                stacks.push("".to_owned());
            }
        }

        for (i, c) in row.into_iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(c);
            }
        }
    }

    for stack in &mut stacks {
        *stack = stack.chars().rev().collect();
    }

    let mut procedure = vec![];
    for line in lines.by_ref() {
        let line = line?;
        procedure.push(line.parse()?);
    }

    Ok(Input { stacks, procedure })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                .skip(1)
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), "CMZ".to_owned());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), "MCD".to_owned());
        Ok(())
    }
}
