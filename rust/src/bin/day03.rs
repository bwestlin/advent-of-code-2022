use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<String>;

fn prio(c: char) -> i32 {
    (match c {
        ('a'..='z') => c as u8 - b'a' + 1,
        ('A'..='Z') => c as u8 - b'A' + 27,
        _ => unreachable!(),
    }) as i32
}

fn part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|rucksack| {
            let (a, b) = rucksack.split_at(rucksack.len() / 2);

            let mut found = None;
            for c in a.chars() {
                if b.contains(c) {
                    found = Some(c);
                    break;
                }
            }

            found.map(prio).unwrap_or(0)
        })
        .sum()
}

fn part2(input: &Input) -> i32 {
    input
        .chunks(3)
        .map(|groups| {
            let mut buffer = groups[0].clone();
            for group in groups.iter().take(3).skip(1) {
                let mut next_buffer = String::with_capacity(buffer.len());
                for c in buffer.chars() {
                    if group.contains(c) {
                        next_buffer.push(c);
                    }
                }
                buffer = next_buffer;
            }

            buffer.chars().next().map(prio).unwrap_or(0)
        })
        .sum()
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
    let path = env::args()
        .nth(1)
        .with_context(|| "No input file given".to_owned())?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

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
        assert_eq!(part1(&as_input(INPUT)?), 157);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 70);
        Ok(())
    }
}
