use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Round>;

#[derive(Debug)]
struct Round {
    opp: Shape,
    strat: Strategy,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Strategy {
    X,
    Y,
    Z,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn is_win(&self, other: &Shape) -> bool {
        *self == other.win()
    }

    fn loose(&self) -> Shape {
        match self {
            Self::Rock => Shape::Scissors,
            Self::Paper => Shape::Rock,
            Self::Scissors => Shape::Paper,
        }
    }

    fn draw(&self) -> Shape {
        *self
    }

    fn win(&self) -> Shape {
        match self {
            Self::Rock => Shape::Paper,
            Self::Paper => Shape::Scissors,
            Self::Scissors => Shape::Rock,
        }
    }
}

fn solve<F>(input: &Input, mut strat_fn: F) -> u32
where
    F: FnMut(&Shape, &Strategy) -> Shape,
{
    input
        .iter()
        .map(|Round { opp, strat }| {
            let you = strat_fn(opp, strat);

            let score = if *opp == you {
                3
            } else {
                6 * you.is_win(opp) as u32
            };
            you.score() + score
        })
        .sum()
}

fn part1(input: &Input) -> u32 {
    solve(input, |_opp, strat| match strat {
        Strategy::X => Shape::Rock,
        Strategy::Y => Shape::Paper,
        Strategy::Z => Shape::Scissors,
    })
}

fn part2(input: &Input) -> u32 {
    solve(input, |opp, strat| match strat {
        Strategy::X => opp.loose(),
        Strategy::Y => opp.draw(),
        Strategy::Z => opp.win(),
    })
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for Round {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split_whitespace();

        let opp = match i.next() {
            Some("A") => Shape::Rock,
            Some("B") => Shape::Paper,
            Some("C") => Shape::Scissors,
            s => anyhow::bail!("Unknown opponent {:?}", s),
        };

        let strat = match i.next() {
            Some("X") => Strategy::X,
            Some("Y") => Strategy::Y,
            Some("Z") => Strategy::Z,
            s => anyhow::bail!("Unknown strategy {:?}", s),
        };

        Ok(Round { opp, strat })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader.lines().map(|line| line?.parse::<Round>()).collect()
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
        A Y
        B X
        C Z";

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
        assert_eq!(part1(&as_input(INPUT)?), 15);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 12);
        Ok(())
    }
}
