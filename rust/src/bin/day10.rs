use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Insruction>;

#[derive(Debug)]
enum Insruction {
    Addx(i32),
    Noop,
}

#[derive(Debug)]
struct Cpu {
    register: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 0,
        }
    }

    fn execute(&mut self, ins: &Insruction) {
        match ins {
            Insruction::Addx(value) => {
                self.register += value;
                self.cycle += 2;
            }
            Insruction::Noop => {
                self.cycle += 1;
            }
        }
    }
}

#[derive(Debug)]
struct Crt {
    pixels: [[bool; 40]; 6],
    last_cycle: usize,
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: [[false; 40]; 6],
            last_cycle: 0,
        }
    }

    fn draw(&mut self, pos: i32, cycle: usize) {
        for i in self.last_cycle..cycle {
            let x = i % 40;
            let y = i / 40;
            let ix = i % 40;
            let lit = (ix as i32 - pos).abs() <= 1;
            self.pixels[y][x] = lit;
        }
        self.last_cycle = cycle;
    }

    fn print(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", if self.pixels[y][x] { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn part1(input: &Input) -> i32 {
    let mut cpu = Cpu::new();

    let capture_points = [20, 60, 100, 140, 180, 220];
    let mut captured = vec![];

    for ins in input {
        let prev_register = cpu.register;
        cpu.execute(ins);

        if captured.len() < capture_points.len() && cpu.cycle >= capture_points[captured.len()] {
            captured.push(prev_register);
        }
    }

    captured
        .into_iter()
        .zip(capture_points.into_iter())
        .map(|(a, b)| a * b as i32)
        .sum()
}

fn part2(input: &Input) {
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();

    for ins in input {
        let prev_register = cpu.register;
        cpu.execute(ins);
        crt.draw(prev_register, cpu.cycle);
    }

    crt.print();
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2:");
        part2(&input);
        Ok(())
    })
}

impl FromStr for Insruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let parts = parts.by_ref();

        Ok(match (parts.next(), parts.next()) {
            (Some("addx"), Some(value)) => Insruction::Addx(value.parse::<i32>()?),
            (Some("noop"), None) => Insruction::Noop,
            _ => anyhow::bail!("Unknown instruction: {}", s),
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map(|line| line?.parse::<Insruction>())
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
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";

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
        assert_eq!(part1(&as_input(INPUT)?), 13140);
        Ok(())
    }
}
