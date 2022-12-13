use std::cmp::Ordering;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Pair>;

#[derive(Debug)]
struct Pair {
    left: Value,
    right: Value,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Value {
    fn append(&mut self, value: Value) {
        match self {
            Value::Integer(_) => panic!("Can't append Integer"),
            Value::List(list) => list.push(value),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", v),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum CmpResult {
    CorrectOrder,
    IncorrectOrder,
    Continue,
}

fn check_order(left: &Value, right: &Value) -> CmpResult {
    match (left, right) {
        (Value::Integer(l), Value::Integer(r)) => match l.cmp(r) {
            Ordering::Less => CmpResult::CorrectOrder,
            Ordering::Equal => CmpResult::Continue,
            Ordering::Greater => CmpResult::IncorrectOrder,
        },
        (Value::Integer(_), Value::List(_)) => check_order(&Value::List(vec![left.clone()]), right),
        (Value::List(_), Value::Integer(_)) => check_order(left, &Value::List(vec![right.clone()])),
        (Value::List(l), Value::List(r)) => {
            for i in 0..(std::cmp::max(l.len(), r.len())) {
                if i >= l.len() && l.len() != r.len() {
                    return CmpResult::CorrectOrder;
                }
                if i >= r.len() && l.len() != r.len() {
                    return CmpResult::IncorrectOrder;
                }

                let c = check_order(&l[i], &r[i]);
                if c != CmpResult::Continue {
                    return c;
                }
            }
            CmpResult::Continue
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut idxs = vec![];

    for (idx, Pair { left, right }) in input.iter().enumerate() {
        match check_order(left, right) {
            CmpResult::CorrectOrder => idxs.push(idx + 1),
            CmpResult::IncorrectOrder => {}
            CmpResult::Continue => {
                unreachable!()
            }
        }
    }

    idxs.into_iter().sum()
}

fn part2(input: &Input) -> usize {
    let mut packets = vec![];
    for Pair { left, right } in input {
        packets.push(left);
        packets.push(right);
    }
    let dp1 = "[[2]]".parse::<Value>().unwrap();
    let dp2 = "[[6]]".parse::<Value>().unwrap();
    packets.push(&dp1);
    packets.push(&dp2);

    packets.sort_by(|a, b| match check_order(a, b) {
        CmpResult::CorrectOrder => Ordering::Less,
        CmpResult::IncorrectOrder => Ordering::Greater,
        CmpResult::Continue => panic!("Unable to sort packets!"),
    });

    [&dp1, &dp2]
        .into_iter()
        .flat_map(|dp| {
            packets
                .iter()
                .enumerate()
                .find(|(_, &p)| p == dp)
                .map(|(i, _)| i + 1)
        })
        .product()
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

impl FromStr for Value {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];

        let mut idx = 0;
        while idx < s.len() {
            match &s[idx..idx + 1] {
                "[" => {
                    stack.push(Value::List(vec![]));
                    idx += 1;
                }
                "]" => {
                    if stack.len() > 1 {
                        let top = stack.pop().unwrap();
                        let last = stack.len() - 1;
                        stack[last].append(top);
                    }
                    idx += 1;
                }
                "," => {
                    idx += 1;
                }
                _ => {
                    let s = &s[idx..]
                        .chars()
                        .take_while(|&c| ('0'..='9').contains(&c))
                        .collect::<String>();

                    let v = s.parse::<u8>()?;
                    let last = stack.len() - 1;
                    stack[last].append(Value::Integer(v));
                    idx += s.len();
                }
            }
        }

        let root = stack.pop().unwrap();

        Ok(root)
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines();
    let lines = lines.by_ref();

    let mut pairs = vec![];

    loop {
        let line = lines.next();
        let left = line.unwrap()?.parse()?;

        let line = lines.next();
        let right = line.unwrap()?.parse()?;

        pairs.push(Pair { left, right });

        let line = lines.next();
        if line.is_none() {
            break;
        }
        line.unwrap()?;
    }

    Ok(pairs)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]";

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
        assert_eq!(part2(&as_input(INPUT)?), 140);
        Ok(())
    }
}
