use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{prelude::*, Lines};

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Monkey>;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_div: u64,
    false_to: usize,
    true_to: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Plus(u64),
    Multiply(u64),
    Square,
}

fn solve<F>(mut monkeys: Vec<Monkey>, rounds: usize, manage_worry_level_fn: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut inspect_counts = vec![0; monkeys.len()];
    let mut throws_buf = vec![];

    for _ in 0..rounds {
        for (m_idx, inspect_count) in inspect_counts.iter_mut().enumerate() {
            let monkey = monkeys.get_mut(m_idx).unwrap();
            throws_buf.clear();

            while let Some(worry_level) = monkey.items.pop_front() {
                *inspect_count += 1;

                let new_worry_level = match monkey.operation {
                    Operation::Plus(value) => worry_level + value,
                    Operation::Multiply(value) => worry_level * value,
                    Operation::Square => worry_level * worry_level,
                };

                let new_worry_level = manage_worry_level_fn(new_worry_level);

                let is_devisable = new_worry_level % monkey.test_div == 0;

                let target = if is_devisable {
                    monkey.true_to
                } else {
                    monkey.false_to
                };

                throws_buf.push((target, new_worry_level));
            }

            for (target, worry_level) in throws_buf.iter() {
                monkeys[*target].items.push_back(*worry_level);
            }
        }
    }

    inspect_counts.sort();
    inspect_counts.into_iter().rev().take(2).product()
}

fn part1(input: &Input) -> u64 {
    solve(input.clone(), 20, |worry_level| worry_level / 3)
}

fn part2(input: &Input) -> u64 {
    let monkey_div_lcm = input
        .iter()
        .skip(1)
        .fold(input[0].test_div, |acc, monkey| lcm(acc, monkey.test_div));

    solve(input.clone(), 10000, |worry_level| {
        worry_level % monkey_div_lcm
    })
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if a == b || b == 0 {
            break a;
        } else if a == 0 {
            break b;
        } else if b > a {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl Monkey {
    fn read_input<R: Read>(lines: &mut Lines<BufReader<R>>) -> Result<Monkey> {
        let mut next = || {
            let line = lines.next();
            let line = line.context("Expected line")?;
            Ok::<String, anyhow::Error>(line?)
        };
        next()?;
        let items = next()?
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();

        let operation = match next()?
            .split('=')
            .nth(1)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["old", "*", "old"] => Operation::Square,
            ["old", "+", s] => Operation::Plus(s.parse().unwrap()),
            ["old", "*", s] => Operation::Multiply(s.parse().unwrap()),
            _ => anyhow::bail!("Unknown operation"),
        };

        let test_div = next()?.split_ascii_whitespace().last().unwrap().parse()?;

        let true_to = next()?.split_ascii_whitespace().last().unwrap().parse()?;

        let false_to = next()?.split_ascii_whitespace().last().unwrap().parse()?;

        Ok(Self {
            items,
            operation,
            test_div,
            false_to,
            true_to,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines();
    let lines = lines.by_ref();

    let mut monkeys = vec![];
    loop {
        let monkey = Monkey::read_input(lines)?;
        monkeys.push(monkey);

        let line = lines.next();
        if let Some(line) = line {
            line?;
        } else {
            break;
        }
    }

    Ok(monkeys)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

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
        assert_eq!(part1(&as_input(INPUT)?), 10605);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 2713310158);
        Ok(())
    }
}
