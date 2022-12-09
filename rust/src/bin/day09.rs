use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Move>;

#[derive(Debug)]
struct Move {
    dir: Direction,
    num: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

#[derive(Debug)]
struct Rope {
    parts: Vec<Pos>,
}

impl Rope {
    fn new(len: usize, start_pos: Pos) -> Self {
        let parts = vec![start_pos; len];
        Self { parts }
    }

    fn move_head(&mut self, dir: &Direction) {
        let parts = &mut self.parts;
        parts[0].step(dir);

        for i in 0..(parts.len() - 1) {
            let head = parts[i];
            let mut tail = parts[i + 1];

            let dx = head.x - tail.x;
            let dy = head.y - tail.y;
            if dx.abs() > 1 || dy.abs() > 1 {
                if dx == 0 {
                    tail.y += dy.signum();
                } else if dy == 0 {
                    tail.x += dx.signum();
                } else {
                    tail.y += dy.signum();
                    tail.x += dx.signum();
                }
            }

            parts[i + 1] = tail;
        }
    }

    fn tail(&self) -> Pos {
        self.parts[self.parts.len() - 1]
    }
}

fn solve(input: &Input, len: usize) -> usize {
    let start = Pos { x: 0, y: 0 };
    let mut rope = Rope::new(len, start);

    let mut tail_visited = HashSet::new();
    tail_visited.insert(rope.tail());

    for Move { dir, num } in input {
        for _ in 0..*num {
            rope.move_head(dir);
            tail_visited.insert(rope.tail());
        }
    }

    tail_visited.len()
}

fn part1(input: &Input) -> usize {
    solve(input, 2)
}

fn part2(input: &Input) -> usize {
    solve(input, 10)
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => anyhow::bail!("Unknown direction {}", s),
        })
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let split = split.by_ref();
        Ok(Move {
            dir: split.next().context("No direction")?.parse::<Direction>()?,
            num: split.next().context("No num")?.parse::<usize>()?,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map(|line| line?.parse::<Move>())
        .collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

    const INPUT2: &str = "
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";

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
        assert_eq!(part1(&as_input(INPUT)?), 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 1);
        assert_eq!(part2(&as_input(INPUT2)?), 36);
        Ok(())
    }
}
